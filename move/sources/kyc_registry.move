/// KYC Registry Module
/// Manages KYC document records on Sui blockchain
module walrus_compliance::kyc_registry {
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;
    use sui::event;
    use std::string::{Self, String};

    /// KYC Document record stored on-chain
    struct KycDocument has key, store {
        id: UID,
        user_id: String,           // Off-chain user identifier
        blob_id: String,           // Walrus blob ID
        document_hash: String,     // SHA-256 hash of document
        document_type: String,     // passport, driver_license, etc.
        uploaded_at: u64,          // Timestamp
        verification_status: u8,   // 0=pending, 1=approved, 2=rejected
        verified_at: u64,          // Timestamp of verification
        verifier: address,         // Address of verifier (if verified)
    }

    /// Event emitted when a KYC document is registered
    struct KycDocumentRegistered has copy, drop {
        document_id: address,
        user_id: String,
        blob_id: String,
        document_hash: String,
    }

    /// Event emitted when KYC status is updated
    struct KycStatusUpdated has copy, drop {
        document_id: address,
        user_id: String,
        old_status: u8,
        new_status: u8,
        verifier: address,
    }

    /// Error codes
    const E_UNAUTHORIZED: u64 = 1;
    const E_INVALID_STATUS: u64 = 2;
    const E_ALREADY_VERIFIED: u64 = 3;

    /// Register a new KYC document
    public entry fun register_document(
        user_id: vector<u8>,
        blob_id: vector<u8>,
        document_hash: vector<u8>,
        document_type: vector<u8>,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        let document = KycDocument {
            id: object::new(ctx),
            user_id: string::utf8(user_id),
            blob_id: string::utf8(blob_id),
            document_hash: string::utf8(document_hash),
            document_type: string::utf8(document_type),
            uploaded_at: tx_context::epoch(ctx),
            verification_status: 0, // Pending
            verified_at: 0,
            verifier: sender,
        };

        let document_id = object::uid_to_address(&document.id);

        event::emit(KycDocumentRegistered {
            document_id,
            user_id: document.user_id,
            blob_id: document.blob_id,
            document_hash: document.document_hash,
        });

        // Transfer to the user who registered it
        transfer::transfer(document, sender);
    }

    /// Update KYC verification status (only by authorized verifier)
    public entry fun update_verification_status(
        document: &mut KycDocument,
        new_status: u8,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);

        // In production, you'd check if sender is an authorized verifier
        // For now, we'll allow the owner to update
        assert!(new_status <= 2, E_INVALID_STATUS);

        let old_status = document.verification_status;
        document.verification_status = new_status;
        document.verified_at = tx_context::epoch(ctx);
        document.verifier = sender;

        event::emit(KycStatusUpdated {
            document_id: object::uid_to_address(&document.id),
            user_id: document.user_id,
            old_status,
            new_status,
            verifier: sender,
        });
    }

    /// Get document hash (for verification)
    public fun get_document_hash(document: &KycDocument): String {
        document.document_hash
    }

    /// Get blob ID
    public fun get_blob_id(document: &KycDocument): String {
        document.blob_id
    }

    /// Get verification status
    public fun get_verification_status(document: &KycDocument): u8 {
        document.verification_status
    }

    /// Check if document is verified (approved)
    public fun is_verified(document: &KycDocument): bool {
        document.verification_status == 1
    }

    #[test_only]
    public fun test_init(ctx: &mut TxContext) {
        // Test initialization
    }
}
