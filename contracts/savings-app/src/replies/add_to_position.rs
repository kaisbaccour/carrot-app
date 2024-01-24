use abstract_sdk::AbstractResponse;
use cosmwasm_std::{Binary, DepsMut, Env, Reply};
use osmosis_std::types::cosmos::authz::v1beta1::MsgExecResponse;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgAddToPositionResponse;

use crate::helpers::get_user;
use crate::state::POSITION;
use crate::{
    contract::{App, AppResult},
    state::Position,
};

pub fn add_to_position_reply(deps: DepsMut, _env: Env, app: App, reply: Reply) -> AppResult {
    // Parse the msg exec response from the reply
    let authz_response: MsgExecResponse = reply.result.try_into()?;

    // Parse the position response from the first authz message
    let response: MsgAddToPositionResponse =
        Binary(authz_response.results[0].clone()).try_into()?;

    // We get the recipient of the position
    let recipient = get_user(deps.as_ref(), &app)?;

    // We update the position
    let position = Position {
        owner: recipient.clone(),
        position_id: response.position_id,
    };

    POSITION.save(deps.storage, &position)?;

    Ok(app
        .response("create_position_reply")
        .add_attribute("updated_position_id", response.position_id.to_string()))
}
