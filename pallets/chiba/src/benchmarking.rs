#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::mock::last_event;
use crate::Pallet as Chiba;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    set_curator {
        let curator: T::AccountId = whitelisted_caller();
    }: set_curator(RawOrigin::Root, curator.clone())
    verify {
        assert_eq!(curator, Chiba::<T>::curator());
    }

    create_collection {
        let caller: T::AccountId = whitelisted_caller();
    }: create_collection(RawOrigin::Signed(caller.clone()), Vec::<u8>::default(), ClassData::default())
    verify {
        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::CollectionCreated(0)),
        );
    }

    mint {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
    }: mint(RawOrigin::Signed(caller.clone()), Default::default(), Vec::<u8>::default(), TokenData::default())
    verify {
        assert_eq!(
            last_event(),
            crate::mock::Event::chiba(crate::RawEvent::TokenMinted(0, 0)),
        );
    }

    // TODO: use non-default balance to appreciate
    appreciate {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
    }: appreciate(RawOrigin::Signed(caller.clone()), Default::default(), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::TokenAppreciated(0, 0, 0)),
        );
    }

    toggle_display {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
    }: toggle_display(RawOrigin::Signed(caller.clone()), Default::default(), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::TokenDisplayToggled(0, 0, false)),
        );
    }

    transfer {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
    }: transfer(RawOrigin::Signed(caller.clone()), Default::default(), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::TokenTransferred(0, 0, 0)),
        );
    }

    // TODO: where is AccountId in event coming from
    create_offer {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
        let offerer: T::AccountId = account("offerer", 0, 0);
    }: create_offer(RawOrigin::Signed(offerer.clone()), Default::default(), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::OfferCreated(0, 0, 0, 15504002658165612567)),
        );
    }

    // TODO: where are AccountIds in event coming from
    accept_offer {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
        let offerer: T::AccountId = account("offerer", 0, 0);
        Chiba::<T>::create_offer(
            <T as frame_system::Config>::Origin::from(RawOrigin::Signed(offerer.clone())),
            Default::default(),
            Default::default(),
            Default::default()
        )?;
    }: accept_offer(RawOrigin::Signed(caller.clone()), Default::default(), Default::default(), offerer)
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::OfferAccepted(0, 0, 15276289921735352792, 15504002658165612567)),
        );
    }


    // TODO: where are AccountIds in event coming from
    cancel_offer {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
        let offerer: T::AccountId = account("offerer", 0, 0);
        Chiba::<T>::create_offer(
            <T as frame_system::Config>::Origin::from(RawOrigin::Signed(offerer.clone())),
            Default::default(),
            Default::default(),
            Default::default()
        )?;
    }: cancel_offer(RawOrigin::Signed(offerer.clone()), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::OfferCanceled(0, 0, 15276289921735352792, 15504002658165612567)),
        );
    }

    report {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
        let reporter: T::AccountId = account("reporter", 0, 0);
    }: report(RawOrigin::Signed(reporter.clone()), Default::default(), Default::default(), ReportReason::None)
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::ReportReceived(0, 0, ReportReason::None)),
        );
    }

    accept_report {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
        let reporter: T::AccountId = account("reporter", 0, 0);
        Chiba::<T>::report(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(reporter.clone())), Default::default(), Default::default(), ReportReason::None)?;
        let curator: T::AccountId = account("curator", 0, 0);
        Chiba::<T>::set_curator(<T as frame_system::Config>::Origin::from(RawOrigin::Root), curator.clone())?;
    }: accept_report(RawOrigin::Signed(curator.clone()), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::ReportAccepted(0, 0)),
        );
    }

    clear_report {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
        let reporter: T::AccountId = account("reporter", 0, 0);
        Chiba::<T>::report(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(reporter.clone())), Default::default(), Default::default(), ReportReason::None)?;
        let curator: T::AccountId = account("curator", 0, 0);
        Chiba::<T>::set_curator(<T as frame_system::Config>::Origin::from(RawOrigin::Root), curator.clone())?;
    }: clear_report(RawOrigin::Signed(curator.clone()), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::ReportCleared(0, 0)),
        );
    }

    burn {
        let caller: T::AccountId = whitelisted_caller();
        Chiba::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default())?;
        Chiba::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default())?;
    }: burn(RawOrigin::Signed(caller.clone()), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::chiba(crate::RawEvent::TokenBurned(0, 0)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use frame_support::assert_ok;

    #[test]
    fn set_curator() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_set_curator::<Test>());
        });
    }

    #[test]
    fn create_collection() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_create_collection::<Test>());
        });
    }

    #[test]
    fn mint() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_mint::<Test>());
        });
    }

    #[test]
    fn appreciate() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_appreciate::<Test>());
        });
    }

    #[test]
    fn toggle_display() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_toggle_display::<Test>());
        });
    }

    #[test]
    fn transfer() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_transfer::<Test>());
        });
    }

    #[test]
    fn create_offer() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_create_offer::<Test>());
        });
    }

    #[test]
    fn accept_offer() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_accept_offer::<Test>());
        });
    }

    #[test]
    fn cancel_offer() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_cancel_offer::<Test>());
        });
    }

    #[test]
    fn report() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_report::<Test>());
        });
    }

    #[test]
    fn accept_report() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_accept_report::<Test>());
        });
    }

    #[test]
    fn clear_report() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_clear_report::<Test>());
        });
    }

    #[test]
    fn burn() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_burn::<Test>());
        });
    }
}
