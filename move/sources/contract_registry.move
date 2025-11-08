/// Contract Registry Module
/// Manages electronic contracts and signatures on Sui blockchain
module walrus_compliance::contract_registry {
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;
    use sui::event;
    use sui::vec_map::{Self, VecMap};
    use std::string::{Self, String};
    use std::vector;

    /// Contract record stored on-chain
    struct Contract has key, store {
        id: UID,
        title: String,
        blob_id: String,              // Walrus blob ID of contract document
        document_hash: String,        // SHA-256 hash of contract
        created_at: u64,              // Timestamp
        created_by: address,          // Creator address
        required_signers: vector<address>,  // List of required signers
        signatures: VecMap<address, Signature>,  // Signer -> Signature mapping
        status: u8,                   // 0=draft, 1=pending, 2=signed, 3=expired
    }

    /// Signature record
    struct Signature has store, copy, drop {
        signer: address,
        signature_hash: String,       // Hash of signature data
        signed_at: u64,               // Timestamp
    }

    /// Event emitted when a contract is created
    struct ContractCreated has copy, drop {
        contract_id: address,
        title: String,
        blob_id: String,
        document_hash: String,
        created_by: address,
    }

    /// Event emitted when a contract is signed
    struct ContractSigned has copy, drop {
        contract_id: address,
        signer: address,
        signature_hash: String,
        signed_at: u64,
        all_signed: bool,
    }

    /// Error codes
    const E_UNAUTHORIZED: u64 = 1;
    const E_NOT_REQUIRED_SIGNER: u64 = 2;
    const E_ALREADY_SIGNED: u64 = 3;
    const E_INVALID_STATUS: u64 = 4;

    /// Create a new contract
    public entry fun create_contract(
        title: vector<u8>,
        blob_id: vector<u8>,
        document_hash: vector<u8>,
        required_signers: vector<address>,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        let contract = Contract {
            id: object::new(ctx),
            title: string::utf8(title),
            blob_id: string::utf8(blob_id),
            document_hash: string::utf8(document_hash),
            created_at: tx_context::epoch(ctx),
            created_by: sender,
            required_signers,
            signatures: vec_map::empty(),
            status: 1, // Pending signatures
        };

        let contract_id = object::uid_to_address(&contract.id);

        event::emit(ContractCreated {
            contract_id,
            title: contract.title,
            blob_id: contract.blob_id,
            document_hash: contract.document_hash,
            created_by: sender,
        });

        // Transfer to creator
        transfer::transfer(contract, sender);
    }

    /// Sign a contract
    public entry fun sign_contract(
        contract: &mut Contract,
        signature_hash: vector<u8>,
        ctx: &mut TxContext
    ) {
        let signer = tx_context::sender(ctx);

        // Check if signer is in required_signers list
        assert!(
            vector::contains(&contract.required_signers, &signer),
            E_NOT_REQUIRED_SIGNER
        );

        // Check if already signed
        assert!(
            !vec_map::contains(&contract.signatures, &signer),
            E_ALREADY_SIGNED
        );

        let signature = Signature {
            signer,
            signature_hash: string::utf8(signature_hash),
            signed_at: tx_context::epoch(ctx),
        };

        vec_map::insert(&mut contract.signatures, signer, signature);

        // Check if all required signatures are collected
        let all_signed = vec_map::size(&contract.signatures) ==
                        vector::length(&contract.required_signers);

        if (all_signed) {
            contract.status = 2; // Fully signed
        };

        event::emit(ContractSigned {
            contract_id: object::uid_to_address(&contract.id),
            signer,
            signature_hash: signature.signature_hash,
            signed_at: signature.signed_at,
            all_signed,
        });
    }

    /// Verify a signature exists
    public fun verify_signature(contract: &Contract, signer: address): bool {
        vec_map::contains(&contract.signatures, &signer)
    }

    /// Get contract document hash
    public fun get_document_hash(contract: &Contract): String {
        contract.document_hash
    }

    /// Get contract blob ID
    public fun get_blob_id(contract: &Contract): String {
        contract.blob_id
    }

    /// Check if contract is fully signed
    public fun is_fully_signed(contract: &Contract): bool {
        contract.status == 2
    }

    /// Get number of signatures
    public fun signature_count(contract: &Contract): u64 {
        vec_map::size(&contract.signatures)
    }

    #[test_only]
    public fun test_init(ctx: &mut TxContext) {
        // Test initialization
    }
}
