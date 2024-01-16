/**
* This file was automatically generated by @abstract-money/ts-codegen@0.28.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @abstract-money/ts-codegen generate command to regenerate this file.
*/

import { ExecuteMsg, BaseExecuteMsg, AppExecuteMsg, StdAck, Binary, IbcResponseMsg, Empty, InstantiateMsg, BaseInstantiateMsg, AppInstantiateMsg, AppMigrateMsg, MigrateMsg, BaseMigrateMsg, QueryMsg, BaseQueryMsg, AppQueryMsg } from "./Template.types";
import { CamelCasedProperties } from "type-fest";
export abstract class TemplateExecuteMsgBuilder {
  static base = (baseExecuteMsg: BaseExecuteMsg): ExecuteMsg => {
    return {
      base: baseExecuteMsg
    };
  };
  static module = (appExecuteMsg: AppExecuteMsg): ExecuteMsg => {
    return {
      module: appExecuteMsg
    };
  };
  static ibcCallback = ({
    id,
    msg
  }: CamelCasedProperties<Extract<ExecuteMsg, {
    ibc_callback: unknown;
  }>["ibc_callback"]>): ExecuteMsg => {
    return {
      ibc_callback: ({
        id,
        msg
      } as const)
    };
  };
  static receive = (): ExecuteMsg => {
    return {
      receive: ({} as const)
    };
  };
}
export abstract class TemplateQueryMsgBuilder {
  static base = (baseQueryMsg: BaseQueryMsg): QueryMsg => {
    return {
      base: baseQueryMsg
    };
  };
  static module = (appQueryMsg: AppQueryMsg): QueryMsg => {
    return {
      module: appQueryMsg
    };
  };
}