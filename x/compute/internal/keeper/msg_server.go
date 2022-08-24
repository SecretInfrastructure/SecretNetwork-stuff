package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"

	"github.com/enigmampc/SecretNetwork/x/compute/internal/types"
)

var _ types.MsgServer = msgServer{}

type msgServer struct {
	keeper Keeper
}

func NewMsgServerImpl(k Keeper) types.MsgServer {
	return &msgServer{keeper: k}
}

func (m msgServer) StoreCode(goCtx context.Context, msg *types.MsgStoreCode) (*types.MsgStoreCodeResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	ctx.EventManager().EmitEvent(sdk.NewEvent(
		sdk.EventTypeMessage,
		sdk.NewAttribute(sdk.AttributeKeyModule, types.ModuleName),
		sdk.NewAttribute(sdk.AttributeKeySender, msg.Sender.String()),
	))

	codeID, err := m.keeper.Create(ctx, msg.Sender, msg.WASMByteCode, msg.Source, msg.Builder)

	return &types.MsgStoreCodeResponse{
		CodeID: codeID,
	}, err
}

func (m msgServer) InstantiateContract(goCtx context.Context, msg *types.MsgInstantiateContract) (*types.MsgInstantiateContractResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	ctx.EventManager().EmitEvent(sdk.NewEvent(
		sdk.EventTypeMessage,
		sdk.NewAttribute(sdk.AttributeKeyModule, types.ModuleName),
		sdk.NewAttribute(sdk.AttributeKeySender, msg.Sender.String()),
	))

	contractAddr, data, err := m.keeper.Instantiate(ctx, msg.CodeID, msg.Sender, msg.InitMsg, msg.Label, msg.InitFunds, msg.CallbackSig)

	// we always return data (internally used in reply)
	// note: even if contractAddr == nil then contractAddr.String() is ok
	// \o/🤷🤷‍♂️🤷‍♀️🤦🤦‍♂️🤦‍♀️
	return &types.MsgInstantiateContractResponse{
		Address: contractAddr.String(),
		Data:    data,
	}, err
}

func (m msgServer) ExecuteContract(goCtx context.Context, msg *types.MsgExecuteContract) (*types.MsgExecuteContractResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	ctx.EventManager().EmitEvent(sdk.NewEvent(
		sdk.EventTypeMessage,
		sdk.NewAttribute(sdk.AttributeKeyModule, types.ModuleName),
		sdk.NewAttribute(sdk.AttributeKeySender, msg.Sender.String()),
	))

	data, err := m.keeper.Execute(ctx, msg.Contract, msg.Sender, msg.Msg, msg.SentFunds, msg.CallbackSig)

	// we always return data (internally used in reply)
	return &types.MsgExecuteContractResponse{
		Data: data.Data,
	}, err
}
