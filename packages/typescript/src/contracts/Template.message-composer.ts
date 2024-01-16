/**
* This file was automatically generated by @abstract-money/ts-codegen@0.28.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @abstract-money/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "cosmwasm";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { AppExecuteMsg, AppExecuteMsgFactory } from "@abstract-money/abstract.js";
import { ExecuteMsg, BaseExecuteMsg, AppExecuteMsg, StdAck, Binary, IbcResponseMsg, Empty, InstantiateMsg, BaseInstantiateMsg, AppInstantiateMsg, AppMigrateMsg, MigrateMsg, BaseMigrateMsg, QueryMsg, BaseQueryMsg, AppQueryMsg } from "./Template.types";
export interface TemplateMessage {
  contractAddress: string;
  sender: string;
  base: (baseExecuteMsg: BaseExecuteMsg, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  module: (appExecuteMsg: AppExecuteMsg, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  ibcCallback: ({
    id,
    msg
  }: {
    id: string;
    msg: StdAck;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  receive: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class TemplateMessageComposer implements TemplateMessage {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.base = this.base.bind(this);
    this.module = this.module.bind(this);
    this.ibcCallback = this.ibcCallback.bind(this);
    this.receive = this.receive.bind(this);
  }

  base = (baseExecuteMsg: BaseExecuteMsg, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    const msg = {
      base: {}
    };
    const moduleMsg: AppExecuteMsg<ExecuteMsg> = AppExecuteMsgFactory.executeApp(msg);
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify(moduleMsg)),
        funds: _funds
      })
    };
  };
  module = (appExecuteMsg: AppExecuteMsg, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    const msg = {
      module: {}
    };
    const moduleMsg: AppExecuteMsg<ExecuteMsg> = AppExecuteMsgFactory.executeApp(msg);
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify(moduleMsg)),
        funds: _funds
      })
    };
  };
  ibcCallback = ({
    id,
    msg
  }: {
    id: string;
    msg: StdAck;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    const msg = {
      ibc_callback: {
        id,
        msg
      }
    };
    const moduleMsg: AppExecuteMsg<ExecuteMsg> = AppExecuteMsgFactory.executeApp(msg);
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify(moduleMsg)),
        funds: _funds
      })
    };
  };
  receive = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    const msg = {
      receive: {}
    };
    const moduleMsg: AppExecuteMsg<ExecuteMsg> = AppExecuteMsgFactory.executeApp(msg);
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify(moduleMsg)),
        funds: _funds
      })
    };
  };
}