use
{
    frame_support::dispatch::DispatchResultWithPostInfo
};

impl<T: Config> Pallet<T> 
{
    pub fn burn_tokens(amount: u64) 
    {
        TotalIssuance::<T>::put(TotalIssuance::<T>::get().saturating_sub(amount));
    }
    
    pub fn get_default_take() -> u16 
    {
        return DefaultTake::<T>::get();
    }
    
    pub fn set_default_take(default_take: u16) 
    {
        DefaultTake::<T>::put(default_take);

        Self::deposit_event(Event::DefaultTakeSet(default_take));
    }

    pub fn set_subnet_locked_balance(netuid: u16, amount: u64) 
    {
        SubnetLocked::<T>::insert(netuid, amount);
    }

    pub fn get_subnet_locked_balance(netuid: u16) -> u64 
    {
        return SubnetLocked::<T>::get(netuid);
    }
}

pub trait AssetsInterface<Origin, AssetIdParameter, AccountId, Balance> 
{
	fn force_create(origin: Origin, id: AssetIdParameter, owner: AccountId, is_sufficient: bool, min_balance: Balance) -> DispatchResult;
    fn force_set_metadata(origin: Origin, id: AssetIdParameter, name: Vec<u8>, symbol: Vec<u8>, decimals: u8, is_frozen: bool) -> DispatchResult;
    fn start_destroy(origin: Origin, id: AssetIdParameter) -> DispatchResult;
    fn destroy_accounts(origin: Origin, id: AssetIdParameter) -> DispatchResultWithPostInfo;
    fn destroy_approvals(origin: Origin, id: AssetIdParameter) -> DispatchResultWithPostInfo;
    fn finish_destroy(origin: Origin, id: AssetIdParameter) -> DispatchResult;
	fn mint(origin: Origin, id: AssetIdParameter, beneficiary: AccountId, amount: Balance) -> DispatchResult;
	fn burn(origin: Origin, id: AssetIdParameter, who: AccountId, amount: Balance) -> DispatchResult;
	fn balance(id: AssetIdParameter, who: AccountId) -> Balance;
	fn total_supply(id: AssetIdParameter) -> Balance;
}
pub trait AssetConversionInterface<Origin, AssetId, AssetBalance, AccountId, MaxSwapPathLength>
{
    fn add_liquidity(
        origin:             Origin,
        asset1:             AssetId, 
        asset2:             AssetId, 
        amount1_desired:    AssetBalance, 
        amount2_desired:    AssetBalance, 
        amount1_min:        AssetBalance,
        amount2_min:        AssetBalance,
        mint_to:            AccountId
    ) -> DispatchResult;

    fn create_pool(origin: Origin, asset1: AssetId, asset2: AssetId) -> DispatchResult;

    fn remove_liquidity(
        origin:                 Origin,
        asset1:                 AssetId, 
        asset2:                 AssetId,
        lp_token_burn:          AssetBalance,
        amount1_min_receive:    AssetBalance,
        amount2_min_receive:    AssetBalance,
        withdraw_to:            AccountId
    ) -> DispatchResult;

    fn swap_exact_tokens_for_tokens(
        origin:                 Origin,
        path:                   BoundedVec<AssetId, MaxSwapPathLength>,
        amount_in:              AssetBalance,
        amount_out_min:         AssetBalance,
        send_to:                AccountId,
        keep_alive:             bool 
    ) -> DispatchResult;

    fn swap_tokens_for_exact_tokens(
        origin:                 Origin,
        path:                   BoundedVec<AssetId, MaxSwapPathLength>,
        amount_out:             AssetBalance,
        amount_in_max:          AssetBalance,
        send_to:                AccountId,
        keep_alive:             bool 
    ) -> DispatchResult;
}