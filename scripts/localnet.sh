#!/bin/bash
set -e
echo "Starting local Solana test validator..."
solana-test-validator --reset --quiet &
sleep 5
anchor deploy
anchor test
