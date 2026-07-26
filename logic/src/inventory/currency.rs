use super::*;

pub(super) async fn currency_list(
    db: &SqlitePool,
    player_id: i64,
    currency_ids: Vec<i32>,
) -> Result<GetCurrencyListReply, AppError> {
    let currency_list = currencies::get_currencies(db, player_id, currency_ids).await?;

    Ok(GetCurrencyListReply {
        currency_list: currency_list.into_iter().map(Into::into).collect(),
    })
}

pub(super) async fn exchange_same_currency(
    db: &SqlitePool,
    player_id: i64,
) -> Result<ExchangeSameCurrencyReply, AppError> {
    let poped = currencies::get_poped_exchange_currency_ids(db, player_id).await?;
    let currency_exchanges = config::configs::get()
        .same_currency_exchange
        .iter()
        .map(|row| CurrencyExchangeNo {
            currency_id: Some(row.currency_id),
            quantity: Some(0),
            is_poped: Some(i32::from(poped.contains(&row.currency_id))),
        })
        .collect();

    Ok(ExchangeSameCurrencyReply { currency_exchanges })
}

pub(super) async fn pop_exchange_same_currency(
    db: &SqlitePool,
    player_id: i64,
    currency_ids: Vec<i32>,
) -> Result<PopExchangeSameCurrencyReply, AppError> {
    currencies::mark_exchange_currencies_poped(db, player_id, &currency_ids).await?;

    Ok(PopExchangeSameCurrencyReply { currency_ids })
}

pub(super) async fn exchange_diamond(
    db: &SqlitePool,
    player_id: i64,
    amount: i32,
    op_type: i32,
) -> Result<ExchangeDiamondReply, AppError> {
    if amount <= 0 || !(1..=5).contains(&op_type) {
        return Err(AppError::InvalidRequest);
    }

    let free_limit = config::configs::get()
        .currency
        .get(2)
        .ok_or(AppError::InvalidRequest)?
        .max_limit;
    match currencies::exchange_with_limit(db, player_id, 1, 2, amount, free_limit).await? {
        currencies::LimitedExchangeResult::Applied => {}
        currencies::LimitedExchangeResult::InsufficientSource => {
            return Err(AppError::InsufficientCurrency);
        }
        currencies::LimitedExchangeResult::TargetLimit
        | currencies::LimitedExchangeResult::PurchaseLimit => {
            return Err(AppError::InvalidRequest);
        }
    }

    Ok(ExchangeDiamondReply {
        exchange_diamond: Some(amount),
        op_type: Some(op_type),
    })
}
