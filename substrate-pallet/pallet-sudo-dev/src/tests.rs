
use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{
    test_build, Call, 
    Event as TestEvent, Origin, 
    Sudo as pallet_sudo, Logger as pallet_log,
    SudoCall, LoggerCall, System, Test
};

#[test]
fn test_setup_works() {

    // set root key as 1
    test_build(1).execute_with(|| {
        assert_eq!(pallet_sudo::key(), Some(1u64));
    })
}

#[test]
fn sudo_works() {

    test_build(1).execute_with(|| {
        
        // Call::Balance(), Call::Staking(), 

        let call = Box::new(Call::Logger(LoggerCall::only_sudo_can_call { value: 42, weight: 1_000 }));
        assert_ok!(pallet_sudo::sudo(Origin::signed(1), call));
        assert_eq!(pallet_log::i32_log(), [42i32]);

        let call = Box::new(Call::Logger(LoggerCall::only_sudo_can_call { value: 42, weight: 1_000 }));
        // AccountId: 2 is not a root 
        assert_noop!(pallet_sudo::sudo(Origin::signed(2), call), Error::<Test>::RequireSudo);
    });
}

#[test]
fn sudo_emits_events_works() {

    test_build(1).execute_with(|| {
        // Since event is not emitted on Block number 0
        System::set_block_number(1);

        let call = Box::new(Call::Logger(LoggerCall::only_sudo_can_call { value: 42, weight: 1_000 }));
        assert_ok!(pallet_sudo::sudo(Origin::signed(1), call));
        let sudo_event = TestEvent::Sudo(Event::Sudid { sudo_result: Ok(()) });
        System::assert_has_event(sudo_event);
    });
}

#[test]
fn sudo_unchecked_weight_works() {

    test_build(1).execute_with(|| {

        // Test 1: root calls 
        let logger_call = LoggerCall::only_sudo_can_call { value: 42, weight: 1_000 };
        let call = Box::new(Call::Logger(logger_call));
        assert_ok!(pallet_sudo::sudo_unchecked_weight(Origin::signed(1), call, 1_000));
        assert_eq!(pallet_log::i32_log(), [42]);

        // Test 2: non-root calls
        let logger_call = LoggerCall::only_sudo_can_call { value: 42, weight: 1_000 };
        let call = Box::new(Call::Logger(logger_call));
        assert_noop!(pallet_sudo::sudo_unchecked_weight(Origin::signed(2), call, 1_000), Error::<Test>::RequireSudo);
        assert_eq!(pallet_log::i32_log(), [42]);

        // Test 3: Control the dispatch weight
        // Weight's of 'only_sudo_call' function is '1'
        let logger_call = LoggerCall::only_sudo_can_call { value: 42, weight: 1 };
        let call = Box::new(Call::Logger(logger_call));
        // Set weight of the call to 1_000
        let sudo_unchecked_weight_call = SudoCall::sudo_unchecked_weight { call , weight: 1_000 };
        let dispatch_info = sudo_unchecked_weight_call.get_dispatch_info();
        assert_eq!(dispatch_info.weight, 1_000);
    })
}

#[test]
fn set_key_works() {

    // Test Case 1
    test_build(1).execute_with(|| {
        
        // Calling with root 1 calls 'set_key' should succeed
        assert_ok!(pallet_sudo::set_key(Origin::signed(1), 2));
        // Root key should change
        assert_eq!(pallet_sudo::key(), Some(2));
    });

    // Test Case 2
    test_build(1).execute_with(|| {

        // Calling with non-root 2 'set-key' should fail 
        assert_noop!(pallet_sudo::set_key(Origin::signed(2), 3), Error::<Test>::RequireSudo);
    });
}

#[test]
fn set_key_emits_event_works() {

    test_build(1).execute_with(|| {

        System::set_block_number(1);

        assert_ok!(pallet_sudo::set_key(Origin::signed(1), 2));
        let sudo_set_key_event = TestEvent::Sudo(Event::KeyChanged { old_sudoer: Some(1) });
        System::assert_has_event(sudo_set_key_event);

        assert_ok!(pallet_sudo::set_key(Origin::signed(2), 10));
        let sudo_set_key_event = TestEvent::Sudo(Event::KeyChanged { old_sudoer: Some(2) });
        System::assert_has_event(sudo_set_key_event);
    });
}

#[test]
fn sudo_as_works() {

    test_build(1).execute_with(|| {

        let privileged_logger_call = LoggerCall::only_sudo_can_call { value: 42, weight: 1_000 };
        let call = Box::new(Call::Logger(privileged_logger_call));
        // Origin is root, but not-root '2' will call privileged_logger_call
        // Dispatch call would success but the logic would not work.
        assert_ok!(pallet_sudo::sudo_as(Origin::signed(1), 2, call));
        assert!(pallet_log::i32_log().is_empty());
        assert!(pallet_log::account_log().is_empty());

        let non_privileged_logger_call = LoggerCall::only_signed_can_call { value: 42, weight: 1_000 };
        let call = Box::new(Call::Logger( non_privileged_logger_call ));
        assert_ok!(pallet_sudo::sudo_as(Origin::signed(1), 2, call));
        assert_eq!(pallet_log::i32_log(), [42]);
        assert_eq!(pallet_log::account_log(), [2]);
    });
}