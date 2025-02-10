#set -x
contract_name="tictactoe"
bot_contract_name="ttt_bot"
network="testnet"
#wasm_hash="c5b643adcd569e36c6e24df790747bc3bf98cc0a391804bc00072d4779f542f5"
#contract_id="CD4VZ4L3IHY54TABOIBQOTW36DENW3ZTL5GOZ6EG2FSMTDYLEY5SGV42"
#bot_wasm_hash=""
#bot_contract_id=""

Echo "Building latest..."

# Build
cargo test
stellar contract build

echo "Deploying contracts..."

# Install tic tac toe contract
if [ -z "$wasm_hash" ]; then
    wasm_hash=$(stellar contract install \
    --network $network \
    --source alice \
    --wasm target/wasm32-unknown-unknown/release/${contract_name}.wasm)
    echo "wasm hash: $wasm_hash"
fi

# Deploy tic tac toe contract
if [ -z "$contract_id" ]; then
    contract_id=$(stellar contract deploy \
    --wasm-hash "${wasm_hash}" \
    --source alice \
    --network $network \
    --alias ${contract_name})
    echo "contract id: $contract_id"
fi

# Install bot contract
if [ -z "$bot_wasm_hash" ]; then
    bot_wasm_hash=$(stellar contract install \
    --network $network \
    --source alice \
    --wasm target/wasm32-unknown-unknown/release/${bot_contract_name}.wasm)
    echo "bot wasm hash: $bot_wasm_hash"
fi

# Deploy bot contract
if [ -z "$bot_contract_id" ]; then
    bot_contract_id=$(stellar contract deploy \
    --wasm-hash "${bot_wasm_hash}" \
    --source alice \
    --network $network \
    --alias ${bot_contract_name})
    echo "bot contract id: $bot_contract_id"
fi


# Utility for checking the winner
function check_winner {
  winner=$(stellar contract invoke \
    --id "${contract_id}" \
    --source carol \
    --network $network \
    --send yes \
    -- \
    winner | tr -d '"')
  board=$(stellar contract invoke \
    --id "${contract_id}" \
    --source carol \
    --network $network \
    --send yes \
    -- \
    display)
  echo "Board:"
  echo " ----------- "
  echo $board | jq .[:3] |  awk '{printf "%s", $0}' |  tr -d ' ' | tr 'N"' ' ' | tr ',[]' '|'
  echo
  echo " ----------- "
  echo $board | jq .[3:6] |  awk '{printf "%s", $0}' |  tr -d ' ' | tr 'N"' ' ' | tr ',[]' '|'
  echo
  echo " ----------- "
  echo $board | jq .[6:9] |  awk '{printf "%s", $0}' |  tr -d ' ' | tr 'N"' ' ' | tr ',[]' '|'
  echo
  echo " ----------- "
  if [ -z "${winner}" ]; then
    echo "No winner yet"
  else
    if [ "${winner}" == "${alice}" ]; then echo "You win!" ; fi
    if [ "${winner}" == "${bot_contract_id}" ]; then echo "Bob wins!" ; fi
    exit 0
  fi
}

# Get keys for Alice and Bob
alice=$(stellar keys address alice)


# Bot Game
echo "Game against bot contract"
stellar contract invoke \
  --id "${contract_id}" \
  --source alice \
  --network $network \
  -- \
  start --player_addr $alice --opponent_addr $bot_contract_id
check_winner

while true
do

    read -p "Your move: " player_move && \
        stellar contract invoke \
            --id "${contract_id}" \
            --source alice \
            --network $network \
            --send yes \
            -- \
            play --addr $alice --player_move $player_move
        check_winner
    

    # Let computer go
    echo "Computer's move..."
    stellar contract invoke \
    --id "${bot_contract_id}" \
    --source alice \
    --network $network \
    --send yes \
    -- \
    go --contract "${contract_id}"
    check_winner

    sleep 1
done