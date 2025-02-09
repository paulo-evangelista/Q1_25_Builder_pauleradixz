# NFT Staking Program

This is a program for staking NFTs built on Solana using the Anchor framework. It allows users to lock their NFTs to accumulate points and earn rewards.

## Features

- Initialization of the program's global configuration
- Initialization of user accounts
- NFT Staking
  - NFTs are frozen during the staking period
  - Users accumulate points based on the configuration
- NFT Unstaking
  - NFTs are unfrozen and returned to the user
- Reward Claiming
  - Users can redeem their accumulated points for reward tokens

## Architecture

The program is divided into the following main components:

- `lib.rs`: Entry point that defines public instructions
- `state/`: Definitions of state accounts
  - `StakeConfig`: Global configuration of the program
  - `UserAccount`: Each user's account, stores points and staked amount
  - `StakeAccount`: Represents information of an NFT in staking
- `context/`: Implementation of the logic for each instruction
- `errors.rs`: Definition of program-specific errors

## Instructions

- `initialize_config`: Initializes the program's global configuration
- `initialize_user`: Initializes a user's account
- `stake`: Performs the staking of an NFT
- `unstake`: Removes an NFT from staking
- `claim`: Claims the accumulated rewards

## Flow

1. The administrator initializes the global configuration with `initialize_config`
2. Each user initializes their account with `initialize_user`
3. Users stake NFTs with `stake`
4. Upon staking, users accumulate points based on the configuration
5. Users can withdraw their NFTs from staking with `unstake`
6. Users redeem their accumulated points for rewards using `claim`

## Dependencies

- Anchor Framework
- Solana Program Library (SPL) Token Program
- Metaplex Token Metadata Program

## Configuration

The global configurations include:

- Points earned per staked NFT
- Maximum number of NFTs a user can stake
- Freezing period of NFTs in staking

These are defined during the program's initialization via `initialize_config`.

## Running the Program

To deploy and interact with the program:

1. Set up your Solana and Anchor development environment
2. Run `anchor build` to build the program
3. Run `anchor deploy` to deploy it to the desired cluster
4. Interact with the program using the Anchor Client or Solana CLI
