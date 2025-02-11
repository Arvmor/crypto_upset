// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Life {
    // Genesis Hash which is H256 0x00...00
    address public GOD;
    bytes32 public constant GENESIS_HASH = 0;
    bytes32 public LATEST_HASH = GENESIS_HASH;
    bytes32 public MASK = 0xFFFF000000000000000000000000000000000000000000000000000000000000;

    // Constructor
    constructor() {
        GOD = msg.sender;
    }

    // Function that calculate the keccack256 of genesis hash with a u128 number
    function calculateHash(uint128 number) public view returns (bytes32) {
        return keccak256(abi.encodePacked(LATEST_HASH, number));
    }

    // Function that sets new mask
    function setMask(bytes32 newMask) public {
        require(msg.sender == GOD, "Only GOD can set the mask");

        MASK = newMask;
    }

    // Function that set the new genesis hash
    function setNewHash(uint number) public {
        // Create a new genesis hash
        bytes32 newGenesisHash = calculateHash(uint128(number));

        // if first x..x+n byte of newGenesisHash is not 0, revert
        require((newGenesisHash & MASK) == 0, "Not a valid puzzle solution");

        // Set the new genesis hash
        LATEST_HASH = newGenesisHash;
    }

    // Withdraw the contract balance
    function withdraw() public {
        payable(GOD).transfer(address(this).balance);
    }
}