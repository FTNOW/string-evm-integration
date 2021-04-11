//! # Chiba Studio
//!
//! Kusama's home for creators.
//!
//! ## Overview
//!
//! This pallet builds on top of the ORML NFT pallet and the FRAME Atomic Swap pallet to provide a
//! home for creators in the Kusama Network.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benchmarking;

use codec::{Decode, Encode};
use frame_support::traits::{BalanceStatus, Currency, ExistenceRequirement, Get};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, fail, traits::ReservableCurrency,
};
use frame_system::{ensure_root, ensure_signed};
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::prelude::*;

use orml_nft::{self as nft};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub struct ExtendedInfo {
    pub display_flag: bool,
    pub report: ReportReason,
    pub frozen: bool,
}

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq)]
pub enum ReportReason {
    None,
    Illegal,
    Plagiarism,
    Duplicate,
    Reported,
}

decl_error! {
    pub enum Error for Module<T: Config> {
        CollectionNotFound,
        TokenNotFound,
        OfferNotFound,
        NotTokenOwner,
        NotCollectionOwner,
        NotCollectionOwnerOrCurator,
        NotCurator,
        LowBalance,
        TokenFrozen
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData {
    pub name: Vec<u8>,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TokenData {
    pub name: Vec<u8>,
}

pub trait Config:
    frame_system::Config
    + nft::Config<ClassData = ClassData, TokenData = TokenData>
    + pallet_atomic_swap::Config
{
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Currency: ReservableCurrency<Self::AccountId>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        ClassId = <T as nft::Config>::ClassId,
        TokenId = <T as nft::Config>::TokenId,
        Balance = BalanceOf<T>,
    {
        CollectionCreated(ClassId),
        TokenMinted(ClassId, TokenId),
        TokenAppreciated(ClassId, TokenId, Balance),
        TokenDisplayToggled(ClassId, TokenId, bool),
        TokenTransferred(ClassId, TokenId, AccountId),
        OfferCreated(ClassId, TokenId, Balance, AccountId),
        OfferAccepted(ClassId, TokenId, AccountId, AccountId),
        OfferCanceled(ClassId, TokenId, AccountId, AccountId),
        ReportReceived(ClassId, TokenId, ReportReason),
        ReportAccepted(ClassId, TokenId),
        ReportCleared(ClassId, TokenId),
        TokenBurned(ClassId, TokenId),
    }
);

decl_storage! {
    trait Store for Module<T: Config> as ArtGallery {
        pub Curator get(fn curator): T::AccountId;
        pub TokenExtendedInfo get(fn token_extended_info): double_map
            hasher(twox_64_concat) T::ClassId, hasher(twox_64_concat) T::TokenId => Option<ExtendedInfo>;
        pub Offers get(fn offer): double_map
            hasher(twox_64_concat) (T::ClassId, T::TokenId), hasher(twox_64_concat) T::AccountId => Option<BalanceOf<T>>;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn set_curator(origin, curator: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            Curator::<T>::put(curator);
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn create_collection(origin, metadata: Vec<u8>, class_data: T::ClassData) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let collection_id = nft::Pallet::<T>::create_class(&who, metadata, class_data)?;
            Self::deposit_event(RawEvent::CollectionCreated(collection_id));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn mint(origin,
                collection_id: T::ClassId,
                metadata: Vec<u8>,
                token_data: T::TokenData
            ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let collection = nft::Pallet::<T>::classes(collection_id).ok_or(Error::<T>::CollectionNotFound)?;
            ensure!(collection.owner == who, Error::<T>::NotCollectionOwner);

            //T::Currency::set_lock(PALLET_ID, &who, T::DefaultCost::get(), WithdrawReasons::all());
            // agree there needs to be some cost but I'm not certain it should be via lock since tokens
            // are transferred
            let token_id = nft::Pallet::<T>::mint(&who, collection_id, metadata, token_data)?;
            Self::deposit_event(RawEvent::TokenMinted(collection_id, token_id));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn appreciate(origin,
            collection_id: T::ClassId,
            token_id: T::TokenId,
            amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let token = nft::Pallet::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;
            let balance = T::Currency::free_balance(&who);

            ensure!(balance >= amount, Error::<T>::LowBalance);

            T::Currency::transfer(&who, &token.owner, amount, ExistenceRequirement::AllowDeath)?;
            Self::deposit_event(RawEvent::TokenAppreciated(collection_id, token_id, amount));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn toggle_display(origin,
            collection_id: T::ClassId,
            token_id: T::TokenId,
            display: bool) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let token = nft::Pallet::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;

            ensure!(token.owner == who, Error::<T>::NotTokenOwner);

            let mut info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
                display_flag: false,
                report: ReportReason::None,
                frozen: false
            });

            info.display_flag = display;
            TokenExtendedInfo::<T>::insert(collection_id, token_id, info);
            Self::deposit_event(RawEvent::TokenDisplayToggled(collection_id, token_id, display));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn transfer(origin,
                collection_id: T::ClassId,
                token_id: T::TokenId,
                recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let token = nft::Pallet::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;

            ensure!(token.owner == who, Error::<T>::NotTokenOwner);

            let info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
                display_flag: false,
                report: ReportReason::None,
                frozen: false
            });

            ensure!(info.frozen == false, Error::<T>::TokenFrozen);

            nft::Pallet::<T>::transfer(&who, &recipient, (collection_id, token_id))?;
            Self::deposit_event(RawEvent::TokenTransferred(collection_id, token_id, recipient));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn create_offer(origin,
            collection_id: T::ClassId,
            token_id: T::TokenId,
            price: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            T::Currency::reserve(&who, price)?;
            Offers::<T>::insert((collection_id, token_id), who.clone(), price);
            Self::deposit_event(RawEvent::OfferCreated(collection_id, token_id, price, who));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn accept_offer(origin,
            collection_id: T::ClassId,
            token_id: T::TokenId,
            buyer_address: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let token = nft::Pallet::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;

            ensure!(token.owner == who, Error::<T>::NotTokenOwner);

            let info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
                display_flag: false,
                report: ReportReason::None,
                frozen: false
            });

            ensure!(info.frozen == false, Error::<T>::TokenFrozen);

            if let Some(offer) = Offers::<T>::get((collection_id, token_id), buyer_address.clone()){
                T::Currency::repatriate_reserved(&buyer_address, &who, offer, BalanceStatus::Free)?;
                Offers::<T>::remove((collection_id, token_id), who.clone());
                nft::Pallet::<T>::transfer(&who, &buyer_address, (collection_id, token_id))?;
                Self::deposit_event(RawEvent::OfferAccepted(collection_id, token_id, who, buyer_address));
                Ok(())
            } else {
                fail!(Error::<T>::OfferNotFound);
            }
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn cancel_offer(origin,
            collection_id: T::ClassId,
            token_id: T::TokenId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let token = nft::Pallet::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;

            if let Some(offer) = Offers::<T>::get((collection_id, token_id), who.clone()){
                T::Currency::unreserve(&who, offer);
                Offers::<T>::remove((collection_id, token_id), who.clone());
                Self::deposit_event(RawEvent::OfferCanceled(collection_id, token_id, token.owner, who));
                Ok(())
            } else {
                fail!(Error::<T>::OfferNotFound);
            }
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn report(origin,
            collection_id: T::ClassId,
            token_id: T::TokenId,
            reason: ReportReason) -> DispatchResult {
            ensure_signed(origin)?;
            ensure!(nft::Pallet::<T>::tokens(collection_id, token_id).is_some(), Error::<T>::TokenNotFound);

            let mut info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
                display_flag: false,
                report: ReportReason::None,
                frozen: false
            });

            info.report = reason.clone();
            TokenExtendedInfo::<T>::insert(collection_id, token_id, info);
            Self::deposit_event(RawEvent::ReportReceived(collection_id, token_id, reason));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn accept_report(origin,
            collection_id: T::ClassId,
            token_id: T::TokenId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(nft::Pallet::<T>::tokens(collection_id, token_id).is_some(), Error::<T>::TokenNotFound);
            ensure!(Curator::<T>::get() == who, Error::<T>::NotCurator);

            let mut info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
                display_flag: false,
                report: ReportReason::None,
                frozen: false
            });

            info.report = ReportReason::Reported;
            Self::deposit_event(RawEvent::ReportAccepted(collection_id, token_id));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn clear_report(origin,
            collection_id: T::ClassId,
            token_id: T::TokenId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(nft::Pallet::<T>::tokens(collection_id, token_id).is_some(), Error::<T>::TokenNotFound);
            ensure!(Curator::<T>::get() == who, Error::<T>::NotCurator);

            let mut info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
                display_flag: false,
                report: ReportReason::None,
                frozen: false
            });

            info.report = ReportReason::None;
            Self::deposit_event(RawEvent::ReportCleared(collection_id, token_id));
            Ok(())
        }

        #[weight = T::BlockWeights::get().max_block / 100]
        pub fn burn(origin,
                collection_id: T::ClassId,
                token_id: T::TokenId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let collection = nft::Pallet::<T>::classes(collection_id).ok_or(Error::<T>::CollectionNotFound)?;

            ensure!(Curator::<T>::get() == who || collection.owner == who, Error::<T>::NotCollectionOwnerOrCurator);

            let info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
                display_flag: false,
                report: ReportReason::None,
                frozen: false
            });

            ensure!(info.frozen == false, Error::<T>::TokenFrozen);

            // doesn't make sense - the burn could be by a different person than the lock.
            //T::Currency::remove_lock(PALLET_ID, &who);
            nft::Pallet::<T>::burn(&who, (collection_id, token_id))?;
            Self::deposit_event(RawEvent::TokenBurned(collection_id, token_id));
            Ok(())
        }
    }
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct ChibaSwapAction<T: Config> {
    collection_id: T::ClassId,
    token_id: T::TokenId,
}


// TODO: how are these capabilities actually used?
impl<T: Config> pallet_atomic_swap::SwapAction<<T as frame_system::Config>::AccountId, T>
    for ChibaSwapAction<T>
{
    fn reserve(
        &self,
        source: &<T as frame_system::Config>::AccountId,
    ) -> frame_support::dispatch::DispatchResult {
        if let Some(token) = nft::Pallet::<T>::tokens(self.collection_id, self.token_id) {
            ensure!(token.owner == *source, Error::<T>::NotTokenOwner);

            let mut info = TokenExtendedInfo::<T>::get(self.collection_id, self.token_id)
                .unwrap_or_else(|| ExtendedInfo {
                    display_flag: false,
                    report: ReportReason::None,
                    frozen: false,
                });

            // if token is already frozen, it is already being used in a swap!
            ensure!(info.frozen == false, Error::<T>::TokenFrozen);

            info.frozen = true;
            TokenExtendedInfo::<T>::insert(self.collection_id, self.token_id, info);
            Ok(())
        } else {
            fail!(Error::<T>::TokenNotFound)
        }
    }

    fn claim(
        &self,
        source: &<T as frame_system::Config>::AccountId,
        target: &<T as frame_system::Config>::AccountId,
    ) -> bool {
        if let Some(token) = nft::Pallet::<T>::tokens(self.collection_id, self.token_id) {
            if token.owner == *source {
                nft::Pallet::<T>::transfer(source, target, (self.collection_id, self.token_id))
                    .is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }

    fn weight(&self) -> frame_support::dispatch::Weight {
        //TODO
        T::BlockWeights::get().max_block / 50
    }

    fn cancel(&self, source: &<T as frame_system::Config>::AccountId) {
        if let Some(token) = nft::Pallet::<T>::tokens(self.collection_id, self.token_id) {
            if token.owner == *source {
                let mut info = TokenExtendedInfo::<T>::get(self.collection_id, self.token_id)
                    .unwrap_or_else(|| ExtendedInfo {
                        display_flag: false,
                        report: ReportReason::None,
                        frozen: false,
                    });

                info.frozen = false;
                TokenExtendedInfo::<T>::insert(self.collection_id, self.token_id, info);
            }
        }
    }
}
