// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {UltraVerifier} from "../src/plonk_vk.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import {Test, console} from "forge-std/Test.sol";
import {VmSafe} from "forge-std/Vm.sol";
import {ConvertBytes32ToString} from "./utils/Bytes32ToString.sol";
import "forge-std/console.sol";

contract ZK_ECDSA_Test is Test, ConvertBytes32ToString {
    UltraVerifier verifier;

    function setUp() public {
        verifier = new UltraVerifier();
    }

    function test_shouldSign() public {
        VmSafe.Wallet memory alice = vm.createWallet("alice");
        bytes32 hash = keccak256("Signed by Alice");
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alice.privateKey, hash);
        address signer = ecrecover(hash, v, r, s);
        assertEq(alice.addr, signer);

        bytes memory signature = abi.encodePacked(r, s, v);
        assertEq(ECDSA.recover(hash, signature), alice.addr);

        console.log(alice.publicKeyX);
        console.log(alice.publicKeyY);
        console.logBytes(signature);

        vm.writeFile("data/hash.txt", "");
        vm.writeFile("data/publicKeyX.txt", "");
        vm.writeFile("data/publicKeyY.txt", "");
        vm.writeFile("data/signature.txt", "");

        vm.writeFile("data/hash.txt", bytes32ToString(hash));
        vm.writeFile("data/publicKeyX.txt", bytes32ToString(bytes32(alice.publicKeyX)));
        vm.writeFile("data/publicKeyY.txt", bytes32ToString(bytes32(alice.publicKeyY)));
        vm.writeFile("data/signature.txt", vm.toString(signature));
    }

    function test_shouldVerifyZKP() public view {
        string memory proofFilePath = "./circuits/proofs/ecdsa.proof";
        string memory proof = vm.readLine(proofFilePath);
        bytes memory proofBytes = vm.parseBytes(proof);
        bytes32[] memory publicInputs = new bytes32[](0);

        bool result = verifier.verify(proofBytes, publicInputs);
        assert(result);
    }
}
