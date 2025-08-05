#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame::pallet]
pub mod pallet {
	use super::*;
	use frame::prelude::*;
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Configuration trait for the pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Defines the event type for the pallet.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// Defines the maximum value the counter can hold
		#[pallet::constant]
		type CounterMaxValue: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CounterValueSet { counter_value: u32 },
		CounterIncremented { counter_value: u32, who: T::AccountId, incremented_amount: u32 },
		CounterDecremented { counter_value: u32, who: T::AccountId, decremented_amount: u32 },
	}

	#[pallet::storage]
	pub type CounterValue<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type UserInteractions<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u32>;

	#[pallet::error]
	pub enum Error<T> {
		CounterValueExceedsMax,
		CounterValueBelowZero,
		CounterOverflow,
		UserInteractionOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn set_counter_value(origin: OriginFor<T>, new_value: u32) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(new_value <= T::CounterMaxValue::get(), Error::<T>::CounterValueExceedsMax);

			CounterValue::<T>::put(new_value);

			Self::deposit_event(Event::<T>::CounterValueSet { counter_value: new_value });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn increment(origin: OriginFor<T>, amount_to_increment: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let current_value = CounterValue::<T>::get().unwrap_or(0);

			let new_value = current_value
				.checked_add(amount_to_increment)
				.ok_or(Error::<T>::CounterOverflow)?;

			ensure!(new_value <= T::CounterMaxValue::get(), Error::<T>::CounterValueExceedsMax);

			CounterValue::<T>::put(new_value);

			UserInteractions::<T>::try_mutate(&who, |interactions| -> Result<_, Error<T>> {
				let new_interaction = interactions
					.unwrap_or(0)
					.checked_add(1)
					.ok_or(Error::<T>::UserInteractionOverflow)?;
				*interactions = Some(new_interaction);

				Ok(())
			})?;

			Self::deposit_event(Event::<T>::CounterIncremented {
				counter_value: new_value,
				who,
				incremented_amount: amount_to_increment,
			});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn decrement(origin: OriginFor<T>, amount_to_decrement: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let current_value = CounterValue::<T>::get().unwrap_or(0);

			let new_value = current_value
				.checked_sub(amount_to_decrement)
				.ok_or(Error::<T>::CounterValueBelowZero)?;

			CounterValue::<T>::put(new_value);

			UserInteractions::<T>::try_mutate(&who, |interactions| -> Result<_, Error<T>> {
				let new_interaction = interactions
					.unwrap_or(0)
					.checked_add(1)
					.ok_or(Error::<T>::UserInteractionOverflow)?;
				*interactions = Some(new_interaction);

				Ok(())
			})?;

			Self::deposit_event(Event::<T>::CounterDecremented {
				counter_value: new_value,
				who,
				decremented_amount: amount_to_decrement,
			});

			Ok(())
		}
	}
}
