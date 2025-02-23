<img width="1127" alt="turbin3-gh" src="https://github.com/user-attachments/assets/2323eb4b-642b-4b8c-80ae-5d6e530252ef" />


# Hi there 👋, apaar here 
 ![X (formerly Twitter) Follow](https://img.shields.io/twitter/follow/dotslashapaar)
<br/>

Hey there, welcome to my PoW (Proof of Work)!

I'm a dedicated developer in the Solana ecosystem, recently busy with projects involving Turbin3 and the Q1 Builders Cohort. 

Dive in to explore my latest work, and feel free to leave a comment, open an issue, or even contribute. Let's create something amazing together!

---
##  📚 Projects



<details>
<summary>Vire Protocol (Capstone)</summary>

<br />

# Vire Protocol

**Vire Protocol** (French for "transfer" bc why not xD) is a decentralized tuition payment protocol that leverages blockchain technology to make tuition payments in stablecoins (USDC and USDT) more accessible for international students and universities. Vire Protocol streamlines the payment process by directly minting a unique NFT card for each student, which belongs to the university’s unique mint collection. The protocol automatically freezes (stakes) the NFT at the start of each semester and allows students to unfreeze (unstake) it once the semester ends.

## Table of Contents

- [Overview](#overview)
- [Workflow](#workflow)
- [Protocol Requirements](#protocol-requirements)
  - [Vire Protocol (vire_account)](#vire-protocol-vire_account)
  - [University (uni_account & subject_account)](#university-uni_account)
  - [Student (student_account & student_card_account)](#student-student_account--student_card_account)
- [Features](#features)
- [Getting Started](#getting-started)
- [Future Enhancements](#future-enhancements)
- [Contributing](#contributing)

## Overview

Vire Protocol is designed to address the cumbersome and expensive process of making international tuition payments. By utilizing stablecoins and NFTs on a blockchain network (such as Solana using the Anchor framework), the protocol minimizes intermediaries, reduces fees, and introduces a transparent, secure method for verifying tuition payments.

<img width="674" alt="image" src="https://github.com/user-attachments/assets/d7b2a622-333c-4bcf-a779-3aab2183e1d3" />


## Workflow

1. **Student Payment:**  
   A student pays the tuition fee directly to the university using stablecoins (USDC or USDT).

2. **NFT Card Minting & Freezing:**  
   - Upon successful payment, the protocol mints a unique semester card (NFT) for the student.  
   - This NFT belongs to the university’s unique mint collection.  
   - Immediately after minting, the protocol automatically freezes (stakes) the NFT in the student’s wallet for the duration of the semester.

3. **Semester End & Unfreezing:**  
   - At the end of the semester, the student can unfreeze (unstake) their NFT card, signifying the completion of that semester's tuition payment cycle.

## Protocol Requirements

### Vire Protocol (1 PDA: `vire_account`)

- **Token Deposits:**
  - Allow students to deposit USDT tokens.
  - Allow students to deposit USDC tokens.
- **Fee Management:**
  - Claim a minor fee on each transaction.
  - Maintain a separate vault to store deposit fees.
- **University Partnerships:**
  - Store the number of universities partnered with.

### University (1 PDA: `uni_account`, `sujbect_account`)

- **Identification & Fee Settings:**
  - Assign each university a unique `uni_id`.
  - Allow each university to set a personalized tuition fee.
- **NFT Card Management:**
  - Define a freeze time period for the student NFT card.
  - Set a maximum card (NFT) limit (representing the total number of semesters).
  - Manage a unique mint collection for each university.
- **Enrollment Tracking:**
  - Store the total number of students for the university.

### Student (2 PDAs: `student_account`, `student_card_account`)

- **Identification & NFT Issuance:**
  - Assign each student a unique `student_id`.
  - Issue a semester card (NFT) for every semester paid.
- **Automated NFT Handling:**
  - Automatically freeze (stake) the NFT for the duration of the semester.
  - Allow the student to unfreeze (unstake) the NFT only after the semester has ended.


<img width="560" alt="image" src="https://github.com/user-attachments/assets/a9a0f479-f012-4695-a90b-b7853049697f" />



## Features

- **Direct Stablecoin Payments:**  
  Facilitate tuition payments using USDC and USDT.
  
- **NFT Card Minting:**  
  Mint a unique NFT for each student's semester payment, acting as proof of payment.

- **Automated NFT Staking:**  
  Automatically freeze the NFT upon minting at the start of each semester, and allow unfreezing only after the semester ends.

- **University-Specific Mint Collections:**  
  Each university has its unique mint collection, ensuring that student NFTs are clearly associated with their institution.

- **Fee Collection & Management:**  
  Automatically collect a minor fee per transaction and store it in a dedicated vault.

## Getting Started

### Prerequisites

- **Development Environment:**  
  Node.js and npm/yarn.
- **Blockchain Framework:**  
  Solana CLI and Anchor framework.
- **Programming Language:**  
  TypeScript.

### Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/dotslashapaar/vire-protocol.git
   cd vire-protocol
   ```
   
2. **Install Dependencies:**

    ```bash
    yarn install
    # or
    npm install
    ```

3. **Configure Environment:**
    - Set up your Solana CLI.
    - Configure Anchor by following the Anchor documentation.

4. **Build and Deploy:**

    ```bash
    anchor build
    anchor deploy
    ```

## Future Enhancements
  - **Additional Stablecoin/FIAT Integration:**
    - Extend support to more stablecoins or integrate fiat on/off ramps.

## Contributing

Contributions are welcome! Please fork this repository and submit a pull request with your proposed changes. For major changes, please open an issue first to discuss what you would like to change.


[comment]: <> (Extend Catistics)
</details>

---


<details>
<summary>Prereq - Bridge To Turbin3</summary>


[comment]: <> (Describe)
### Introduction

The "Bridge to Turbin3" course was designed to facilitate a starting point for learning how to use Solana and interact with programs

Key Features

* Creation Keypairs
* Airdroping
* Transfer SOL
* Enroll to Turbin3

Technologies Used

* Solana
* Typescript


[comment]: <> (Extend Catistics)

<br />
</details>

---


<details>
<summary>Prereq - Rust</summary>

[comment]: <> (Describe)
### Introduction

Quite similar to "Bridge to Turbin3" but using Rust instead.

Key Features

* CLI integration
* Creation Keypairs
* Airdroping
* Transfer SOL
* Enroll to Turbin3

Technologies Used

* Solana
* Rust


[comment]: <> (Extend Catistics)

<br />
</details>

---

<details>
<summary>Solana Starter</summary>

<br />

https://github.com/dotslashapaar/TURBIN3-Q1-25/tree/main/solana-starter

[comment]: <> (Extend Catistics)
</details>

---

<details>
<summary>Anchor Vote</summary>

<br />

https://github.com/dotslashapaar/voting-dapp-web3

[comment]: <> (Extend Catistics)
</details>

---

<details>
<summary>Anchor Vault</summary>

<br />

https://github.com/dotslashapaar/anchor-vault

[comment]: <> (Extend Catistics)
</details>

---

<details>
<summary>Anchor Escrow</summary>

<br />

https://github.com/dotslashapaar/anchor-escrow

[comment]: <> (Extend Catistics)
</details>

---

<details>
<summary>Anchor AMM</summary>

<br />

https://github.com/dotslashapaar/automated-market-maker

[comment]: <> (Extend Catistics)
</details>

---

<details>
<summary>Anchor NFT-MarketPlace</summary>

<br />

https://github.com/dotslashapaar/nft-marketplace-anchor

[comment]: <> (Extend Catistics)
</details>

---

<details>
<summary>Anchor NFT-Staking</summary>

<br />

https://github.com/dotslashapaar/Nft-staking-anchor

[comment]: <> (Extend Catistics)
</details>

---

<details>
<summary>Anchor Casino Dice Game</summary>

<br />

https://github.com/dotslashapaar/casino-dice-game-anchor

[comment]: <> (Extend Catistics)
</details>

---

