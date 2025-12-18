use super::{Account, EvmStorageSlot};
use primitives::{Address, HashMap, StorageKey, StorageValue, U256};

/// EVM State is a mapping from addresses to accounts.
pub type EvmState = HashMap<Address, Account>;

/// Structure used for EIP-1153 transient storage
pub type TransientStorage = HashMap<(Address, StorageKey), StorageValue>;

/// An account's Storage is a mapping from 256-bit integer keys to [EvmStorageSlot]s.
pub type EvmStorage = HashMap<StorageKey, EvmStorageSlot>;


/// EvmState that handles incrementing balance lazily to allow for parallel execution.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LazyEvmState {
    /// Loaded state is the state that is loaded from the database.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub loaded_state: EvmState,

    /// Pending balance increments is a map of addresses to the balance increments that are pending.
    /// These addresses are considered touched.
    /// Touching an address that is already in pending_balance_increments is a no-op because it is already considered touched.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub pending_balance_increments: HashMap<Address, U256>,
}