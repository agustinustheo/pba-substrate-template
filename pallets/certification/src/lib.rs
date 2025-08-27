//! # Counter Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Polkadot SDK template
//! as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single block-number
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! To get started with pallet development, consider using this tutorial:
//!
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>
//!
//! And reading the main documentation of the `frame` crate:
//!
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html>
//!
//! And looking at the frame [`kitchen-sink`](https://paritytech.github.io/polkadot-sdk/master/pallet_example_kitchensink/index.html)
//! pallet, a showcase of all pallet macros.
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-counter --open` to view this pallet's documentation.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame::{
    prelude::*,
    traits::Hash,
};
use scale_info::prelude::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html>
// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>
//
// To see a full list of `pallet` macros and their use cases, see:
// <https://paritytech.github.io/polkadot-sdk/master/pallet_example_kitchensink/index.html>
// <https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/index.html>
#[frame::pallet]
pub mod pallet {
    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html>
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: crate::weights::WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Certification struct
    /// Information that is mutable by user
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_storage_derives/index.html>
    #[derive(
        Encode, Decode, TypeInfo, CloneNoBound, PartialEqNoBound, EqNoBound,
    )]
    pub struct Certification<AccountId: Clone + PartialEq + Eq, Hash: Clone + PartialEq + Eq, BlockNumber: Clone + PartialEq + Eq> {
        pub(crate) id: Hash,
        pub(crate) owner_id: AccountId,
        pub(crate) title: Vec<u8>,
        pub(crate) description: Vec<u8>,
        pub(crate) created_at: BlockNumber,
        pub(crate) updated_at: BlockNumber,
    }
    impl<AccountId: Clone + PartialEq + Eq, Hash: Clone + PartialEq + Eq, BlockNumber: Clone + PartialEq + Eq> Certification<AccountId, Hash, BlockNumber> {
        pub(crate) fn new(id: Hash, owner_id: AccountId, title: Vec<u8>, description: Vec<u8>, created_at: BlockNumber, updated_at: BlockNumber) -> Self {
            Self { id, owner_id, title, description, created_at, updated_at }
        }

        pub(crate) fn get_id(&self) -> &Hash {
            &self.id
        }

        pub(crate) fn get_owner_id(&self) -> &AccountId {
            &self.owner_id
        }
    }

    /// The pallet's storage items.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#storage>
    /// <https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/attr.storage.html>
    #[pallet::storage]
    pub type ListOfCertifications<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Certification<T::AccountId, T::Hash, BlockNumberFor<T>>>;

    /// Pallets use events to inform users when important changes are made.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// We usually use passive tense for events.
        CertificationStored {
            who: T::AccountId,
            certification_id: T::Hash,
            created_at: BlockNumberFor<T>,
        },
        CertificationUpdated {
            who: T::AccountId,
            certification_id: T::Hash,
            updated_at: BlockNumberFor<T>,
        },
        CertificationRemoved {
            who: T::AccountId,
            certification_id: T::Hash,
        },
    }

    /// Errors inform users that something went wrong.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::error]
    pub enum Error<T> {
        /// The caller is not the owner of the certification.
        NotOwner,
        /// Certification not found.
        CertificationNotFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#dispatchables>
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn add_certification(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_origin/index.html>
            let who = ensure_signed(origin)?;

            // Convert the u32 into a block number. This is possible because the set of trait bounds
            // defined in [`frame_system::Config::BlockNumber`].
            let block_number: BlockNumberFor<T> = frame_system::Pallet::<T>::block_number();

            // Update storage.
            <ListOfCertifications<T>>::insert(T::Hashing::hash_of(&who), Certification::new(
                T::Hashing::hash_of(&who),
                who.clone(),
                title,
                description,
                block_number,
                block_number,
            ));

            // Emit an event.
            Self::deposit_event(Event::CertificationStored {
                who: who.clone(),
                certification_id: T::Hashing::hash_of(&who),
                created_at: block_number,
            });

            // Return a successful [`DispatchResultWithPostInfo`] or [`DispatchResult`].
            Ok(().into())
        }

        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn update_certification(origin: OriginFor<T>, certification_id: T::Hash, title: Vec<u8>, description: Vec<u8>) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_origin/index.html>
            let who = ensure_signed(origin)?;

            let certification = <ListOfCertifications<T>>::get(certification_id).ok_or(Error::<T>::CertificationNotFound)?;

            ensure!(certification.get_owner_id() == &who, Error::<T>::NotOwner);

            // Convert the u32 into a block number. This is possible because the set of trait bounds
            // defined in [`frame_system::Config::BlockNumber`].
            let block_number: BlockNumberFor<T> = frame_system::Pallet::<T>::block_number();

            // Update storage.
            <ListOfCertifications<T>>::insert(certification_id, Certification::new(
                certification_id,
                who.clone(),
                title,
                description,
                certification.created_at,
                block_number,
            ));

            // Emit an event.
            Self::deposit_event(Event::CertificationUpdated {
                who: who.clone(),
                certification_id,
                updated_at: block_number,
            });

            // Return a successful [`DispatchResultWithPostInfo`] or [`DispatchResult`].
            Ok(().into())
        }

        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn remove_certification(origin: OriginFor<T>, certification_id: T::Hash) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_origin/index.html>
            let who = ensure_signed(origin)?;

            let certification = <ListOfCertifications<T>>::get(certification_id).ok_or(Error::<T>::CertificationNotFound)?;

            ensure!(certification.get_owner_id() == &who, Error::<T>::NotOwner);

            // Remove from storage.
            <ListOfCertifications<T>>::remove(certification_id.clone());

            // Emit an event.
            Self::deposit_event(Event::CertificationRemoved {
                who: who.clone(),
                certification_id,
            });

            // Return a successful [`DispatchResultWithPostInfo`] or [`DispatchResult`].
            Ok(().into())
        }
    }
}
