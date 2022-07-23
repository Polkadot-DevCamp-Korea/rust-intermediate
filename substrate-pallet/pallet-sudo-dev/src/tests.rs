
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