/// Compliance Event Log Module
/// Immutable audit trail for compliance and regulatory requirements
module walrus_compliance::compliance_log {
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;
    use sui::event;
    use std::string::{Self, String};

    /// Compliance event record (immutable)
    struct ComplianceEvent has key, store {
        id: UID,
        event_type: String,        // kyc_upload, contract_signed, document_accessed, etc.
        user_id: String,           // User associated with event
        resource_id: String,       // ID of resource (document, contract, etc.)
        resource_type: String,     // kyc_document, contract, user, system
        action: String,            // Description of action
        metadata: String,          // JSON metadata
        timestamp: u64,            // Event timestamp
        actor: address,            // Address that triggered the event
    }

    /// Event emitted when compliance event is logged
    struct EventLogged has copy, drop {
        event_id: address,
        event_type: String,
        user_id: String,
        resource_id: String,
        timestamp: u64,
        actor: address,
    }

    /// GDPR deletion request record
    struct GdprDeletionRequest has key, store {
        id: UID,
        user_id: String,
        requested_at: u64,
        requested_by: address,
        status: u8,                // 0=pending, 1=completed, 2=rejected
        completed_at: u64,
        notes: String,
    }

    /// Event emitted for GDPR deletion request
    struct GdprRequestCreated has copy, drop {
        request_id: address,
        user_id: String,
        requested_at: u64,
        requested_by: address,
    }

    /// Log a compliance event (immutable record)
    public entry fun log_event(
        event_type: vector<u8>,
        user_id: vector<u8>,
        resource_id: vector<u8>,
        resource_type: vector<u8>,
        action: vector<u8>,
        metadata: vector<u8>,
        ctx: &mut TxContext
    ) {
        let actor = tx_context::sender(ctx);
        let timestamp = tx_context::epoch(ctx);

        let compliance_event = ComplianceEvent {
            id: object::new(ctx),
            event_type: string::utf8(event_type),
            user_id: string::utf8(user_id),
            resource_id: string::utf8(resource_id),
            resource_type: string::utf8(resource_type),
            action: string::utf8(action),
            metadata: string::utf8(metadata),
            timestamp,
            actor,
        };

        let event_id = object::uid_to_address(&compliance_event.id);

        event::emit(EventLogged {
            event_id,
            event_type: compliance_event.event_type,
            user_id: compliance_event.user_id,
            resource_id: compliance_event.resource_id,
            timestamp,
            actor,
        });

        // Make it immutable by transferring to a system address or keeping it
        // For now, transfer to sender for queryability
        transfer::transfer(compliance_event, actor);
    }

    /// Create a GDPR deletion request
    public entry fun create_gdpr_deletion_request(
        user_id: vector<u8>,
        ctx: &mut TxContext
    ) {
        let requester = tx_context::sender(ctx);
        let request = GdprDeletionRequest {
            id: object::new(ctx),
            user_id: string::utf8(user_id),
            requested_at: tx_context::epoch(ctx),
            requested_by: requester,
            status: 0, // Pending
            completed_at: 0,
            notes: string::utf8(b""),
        };

        let request_id = object::uid_to_address(&request.id);

        event::emit(GdprRequestCreated {
            request_id,
            user_id: request.user_id,
            requested_at: request.requested_at,
            requested_by: requester,
        });

        transfer::transfer(request, requester);
    }

    /// Update GDPR deletion request status (admin only in production)
    public entry fun update_gdpr_request_status(
        request: &mut GdprDeletionRequest,
        new_status: u8,
        notes: vector<u8>,
        ctx: &mut TxContext
    ) {
        request.status = new_status;
        request.completed_at = tx_context::epoch(ctx);
        request.notes = string::utf8(notes);
    }

    /// Get event type
    public fun get_event_type(event: &ComplianceEvent): String {
        event.event_type
    }

    /// Get user ID
    public fun get_user_id(event: &ComplianceEvent): String {
        event.user_id
    }

    /// Get timestamp
    public fun get_timestamp(event: &ComplianceEvent): u64 {
        event.timestamp
    }

    #[test_only]
    public fun test_init(ctx: &mut TxContext) {
        // Test initialization
    }
}
