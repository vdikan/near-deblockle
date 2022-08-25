# fill with account Ids:
master_acc=''
first_player_acc=''
second_player_acc=''
game_acc=deblockle-v2.hawthorne.testnet
streaming_acc=streaming-r-v2.dcversus.testnet


echo "<------------------------------------------------------------->"
echo "Delete old contract account."
echo "<------------------------------------------------------------->"
near delete $game_acc $master_acc


echo "<------------------------------------------------------------->"
echo "Create new contract account."
echo "<------------------------------------------------------------->"
near create-account $game_acc --masterAccount $master_acc --initialBalance 10


echo "<------------------------------------------------------------->"
echo "Deploy it to the testnet."
echo "<------------------------------------------------------------->"
near deploy \
    --accountId $game_acc \
    --wasmFile ./out/main.wasm 


echo "<------------------------------------------------------------->"
echo "Add streaming contract"
echo "<------------------------------------------------------------->"
near call $game_acc connect_streaming_contract \
    "{\"streaming_id\": \"$streaming_acc\"}" \
    --accountId $master_acc \
    --gas 300000000000000


echo "<------------------------------------------------------------->"
echo "Register game contract account."
echo "<------------------------------------------------------------->"
near call wrap.testnet storage_deposit \
       "{\"account_id\": \"$game_acc\"}" \
       --accountId $master_acc \
       --depositYocto 12500000000000000000000


echo "<------------------------------------------------------------->"
echo "Deposit wNEAR to first player's account."
echo "<------------------------------------------------------------->"
near call wrap.testnet near_deposit '' \
    --accountId $first_player_acc \
    --deposit 0.3


echo "<------------------------------------------------------------->"
echo "Check game contract status before."
echo "<------------------------------------------------------------->"
near call $game_acc status \
    --accountId $master_acc


echo "<------------------------------------------------------------->"
echo "Register first player."
echo "<------------------------------------------------------------->"
near call wrap.testnet ft_transfer_call \
    '{"receiver_id": "deblockle-v2.hawthorne.testnet", "amount": "300000000000000000000000", "msg": "{\"tokens_per_sec\": \"10000\"}"}' \
    --depositYocto 1 \
    --gas 300000000000000 \
    --accountId $first_player_acc


echo "<------------------------------------------------------------->"
echo "Check game contract status after."
echo "<------------------------------------------------------------->"
near call $game_acc status \
    --accountId $master_acc


echo "<------------------------------------------------------------->"
echo "Deposit wNEAR to second player's account."
echo "<------------------------------------------------------------->"
near call wrap.testnet near_deposit '' \
    --accountId $second_player_acc \
    --deposit 0.3


echo "<------------------------------------------------------------->"
echo "Check game contract status before."
echo "<------------------------------------------------------------->"
near call $game_acc status \
    --accountId $master_acc


echo "<------------------------------------------------------------->"
echo "Register second player."
echo "<------------------------------------------------------------->"
near call wrap.testnet ft_transfer_call \
    '{"receiver_id": "deblockle-v2.hawthorne.testnet", "amount": "300000000000000000000000", "msg":  "{\"tokens_per_sec\": \"10000\"}"}' \
    --depositYocto 1 \
    --gas 300000000000000 \
    --accountId $second_player_acc


echo "<------------------------------------------------------------->"
echo "Check wNEAR contract balance after."
echo "<------------------------------------------------------------->"
near call wrap.testnet ft_balance_of \
    "{\"account_id\": \"$game_acc\"}" \
    --accountId $first_player_acc


echo "<------------------------------------------------------------->"
echo "Check wNEAR contract balance after."
echo "<------------------------------------------------------------->"
near call wrap.testnet ft_balance_of \
    "{\"account_id\": \"$game_acc\"}" \
    --accountId $second_player_acc


echo "<------------------------------------------------------------->"
echo "Check game contract status after."
echo "<------------------------------------------------------------->"
near call $game_acc status \
    --accountId $second_player_acc


# From the response one can see the Ids of streams associated with players.
# They can be used to track statuses to control the streams:
#export s1=
#export s2=
#near call deblockle-v2.hawthorne.testnet start --accountId $master_acc --gas 300000000000000
#near view $streaming_acc get_stream "{\"stream_id\": \"$s2\"}" --accountId $second_player_acc

#############################################################################

echo "<------------------------------------------------------------->"
echo "Start game"
echo "<------------------------------------------------------------->"
  near call $game_acc start --accountId $master_acc --gas 300000000000000 


echo "<------------------------------------------------------------->"
echo "Make first turn: first player"
echo "<------------------------------------------------------------->"
near call $game_acc make_move \
    '{"from_x": 3, "from_y": 5, "to_x": 3, "to_y": 4}' \
    --accountId $first_player_acc \
    --gas 300000000000000

echo "<------------------------------------------------------------->"
echo "Continue turn: first player"
echo "<------------------------------------------------------------->"
near call $game_acc make_move \
    '{"from_x": 3, "from_y": 4, "to_x": 6, "to_y": 4}' \
    --accountId $first_player_acc \
    --gas 300000000000000

echo "<------------------------------------------------------------->"
echo "Make first turn: second player"
echo "<------------------------------------------------------------->"
near call $game_acc make_move \
    '{"from_x": 3, "from_y": 3, "to_x": 3, "to_y": 4}' \
    --accountId $second_player_acc \
    --gas 300000000000000

echo "<------------------------------------------------------------->"
echo "Continue turn: second player"
echo "<------------------------------------------------------------->"
near call $game_acc make_move \
    '{"from_x": 3, "from_y": 4, "to_x": 3, "to_y": 6}' \
    --accountId $second_player_acc \
    --gas 300000000000000

# ...

