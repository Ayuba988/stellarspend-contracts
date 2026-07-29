# Price Oracle Integration

## Overview

The multi-currency wallet and currency conversion contracts use a real on-chain price oracle for defensible pricing. This document describes the oracle architecture, security features, and integration points.

## Architecture

### Components

1. **Oracle Interface** (`contracts/shared/src/oracle.rs`)
   - Defines the `PriceOracle` trait
   - Standardizes price fetching across providers
   - Enables swapping oracle providers

2. **Reflector Oracle Adapter** (`contracts/shared/src/reflector_oracle.rs`)
   - Integrates with Reflector-style oracles
   - Supports TWAP (Time-Weighted Average Price)
   - Handles price staleness checking

3. **Oracle Manager** (`contracts/multi-currency-wallet/src/oracle.rs`)
   - Validates price freshness
   - Checks manipulation resistance
   - Enforces deviation bounds

### Price Flow

let mock_oracle = MockOracle {
    price_value: 1_000_000,
    price_timestamp: env.ledger().timestamp(),
    is_fresh_result: true,
    should_fail: false,
};
