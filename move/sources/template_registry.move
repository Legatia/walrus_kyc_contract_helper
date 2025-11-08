/// Contract Template Registry Module
/// Marketplace for reusable contract templates with variable substitution
module walrus_compliance::template_registry {
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;
    use sui::event;
    use sui::coin::{Self, Coin};
    use sui::sui::SUI;
    use sui::vec_map::{Self, VecMap};
    use std::string::{Self, String};
    use std::vector;

    /// Contract Template NFT (owned by creator)
    struct ContractTemplate has key, store {
        id: UID,
        name: String,
        description: String,
        template_blob_id: String,        // Walrus blob ID of template PDF
        creator: address,
        category: String,                // "mvno", "nda", "employment", etc.
        variables: vector<String>,       // ["{{customer_name}}", "{{plan}}", ...]
        price_per_use: u64,             // Cost in MIST to use this template
        royalty_percentage: u8,          // % of instance fees to creator (0-100)
        usage_count: u64,               // How many times used
        is_public: bool,                // True = marketplace, False = private
        created_at: u64,
        version: u64,                   // For template updates
    }

    /// Contract Instance (derived from template)
    struct ContractInstance has key, store {
        id: UID,
        template_id: address,           // Reference to template
        instance_blob_id: String,       // Walrus blob ID of generated PDF
        variable_data: VecMap<String, String>, // Filled-in values
        created_by: address,
        required_signers: vector<address>,
        signatures: VecMap<address, Signature>,
        status: u8,                     // 0=draft, 1=pending, 2=signed
        payment_tx: address,            // Reference to template payment
        created_at: u64,
    }

    /// Signature record
    struct Signature has store, copy, drop {
        signer: address,
        signature_hash: String,
        signed_at: u64,
    }

    /// Template marketplace listing
    struct TemplateMarketplace has key {
        id: UID,
        templates: vector<address>,     // All public template IDs
        featured: vector<address>,      // Featured templates
    }

    /// Events
    struct TemplateCreated has copy, drop {
        template_id: address,
        name: String,
        creator: address,
        price_per_use: u64,
        is_public: bool,
    }

    struct TemplatePurchased has copy, drop {
        template_id: address,
        buyer: address,
        amount_paid: u64,
    }

    struct InstanceCreated has copy, drop {
        instance_id: address,
        template_id: address,
        created_by: address,
    }

    struct InstanceSigned has copy, drop {
        instance_id: address,
        signer: address,
        fully_signed: bool,
    }

    /// Error codes
    const E_INVALID_ROYALTY: u64 = 1;
    const E_INSUFFICIENT_PAYMENT: u64 = 2;
    const E_NOT_AUTHORIZED: u64 = 3;
    const E_TEMPLATE_NOT_PUBLIC: u64 = 4;
    const E_ALREADY_SIGNED: u64 = 5;

    /// Initialize marketplace (call once at deployment)
    fun init(ctx: &mut TxContext) {
        let marketplace = TemplateMarketplace {
            id: object::new(ctx),
            templates: vector::empty(),
            featured: vector::empty(),
        };
        transfer::share_object(marketplace);
    }

    /// Create a new contract template
    public entry fun create_template(
        name: vector<u8>,
        description: vector<u8>,
        template_blob_id: vector<u8>,
        category: vector<u8>,
        variables: vector<vector<u8>>,
        price_per_use: u64,
        royalty_percentage: u8,
        is_public: bool,
        ctx: &mut TxContext
    ) {
        assert!(royalty_percentage <= 100, E_INVALID_ROYALTY);

        let creator = tx_context::sender(ctx);

        // Convert variables to Strings
        let mut var_strings = vector::empty<String>();
        let mut i = 0;
        while (i < vector::length(&variables)) {
            vector::push_back(&mut var_strings, string::utf8(*vector::borrow(&variables, i)));
            i = i + 1;
        };

        let template = ContractTemplate {
            id: object::new(ctx),
            name: string::utf8(name),
            description: string::utf8(description),
            template_blob_id: string::utf8(template_blob_id),
            creator,
            category: string::utf8(category),
            variables: var_strings,
            price_per_use,
            royalty_percentage,
            usage_count: 0,
            is_public,
            created_at: tx_context::epoch(ctx),
            version: 1,
        };

        let template_id = object::uid_to_address(&template.id);

        event::emit(TemplateCreated {
            template_id,
            name: template.name,
            creator,
            price_per_use,
            is_public,
        });

        // Transfer to creator (they own the template NFT)
        transfer::transfer(template, creator);
    }

    /// Create contract instance from template (with payment)
    public entry fun create_instance_from_template(
        template: &mut ContractTemplate,
        instance_blob_id: vector<u8>,
        variable_keys: vector<vector<u8>>,
        variable_values: vector<vector<u8>>,
        required_signers: vector<address>,
        payment: Coin<SUI>,
        ctx: &mut TxContext
    ) {
        let buyer = tx_context::sender(ctx);

        // Check payment
        let payment_amount = coin::value(&payment);
        assert!(payment_amount >= template.price_per_use, E_INSUFFICIENT_PAYMENT);

        // If template is not public, only creator can use
        if (!template.is_public) {
            assert!(buyer == template.creator, E_TEMPLATE_NOT_PUBLIC);
        };

        // Pay template creator
        transfer::public_transfer(payment, template.creator);

        // Update usage count
        template.usage_count = template.usage_count + 1;

        // Build variable data map
        let mut variable_data = vec_map::empty<String, String>();
        let mut i = 0;
        while (i < vector::length(&variable_keys)) {
            vec_map::insert(
                &mut variable_data,
                string::utf8(*vector::borrow(&variable_keys, i)),
                string::utf8(*vector::borrow(&variable_values, i))
            );
            i = i + 1;
        };

        let instance = ContractInstance {
            id: object::new(ctx),
            template_id: object::uid_to_address(&template.id),
            instance_blob_id: string::utf8(instance_blob_id),
            variable_data,
            created_by: buyer,
            required_signers,
            signatures: vec_map::empty(),
            status: 1, // Pending signatures
            payment_tx: object::uid_to_address(&template.id), // Simplified
            created_at: tx_context::epoch(ctx),
        };

        let instance_id = object::uid_to_address(&instance.id);

        event::emit(TemplatePurchased {
            template_id: object::uid_to_address(&template.id),
            buyer,
            amount_paid: payment_amount,
        });

        event::emit(InstanceCreated {
            instance_id,
            template_id: instance.template_id,
            created_by: buyer,
        });

        transfer::transfer(instance, buyer);
    }

    /// Sign a contract instance
    public entry fun sign_instance(
        instance: &mut ContractInstance,
        signature_hash: vector<u8>,
        ctx: &mut TxContext
    ) {
        let signer = tx_context::sender(ctx);

        // Check if signer is required
        assert!(
            vector::contains(&instance.required_signers, &signer),
            E_NOT_AUTHORIZED
        );

        // Check if already signed
        assert!(
            !vec_map::contains(&instance.signatures, &signer),
            E_ALREADY_SIGNED
        );

        let signature = Signature {
            signer,
            signature_hash: string::utf8(signature_hash),
            signed_at: tx_context::epoch(ctx),
        };

        vec_map::insert(&mut instance.signatures, signer, signature);

        // Check if fully signed
        let fully_signed = vec_map::size(&instance.signatures) ==
                          vector::length(&instance.required_signers);

        if (fully_signed) {
            instance.status = 2; // Fully signed
        };

        event::emit(InstanceSigned {
            instance_id: object::uid_to_address(&instance.id),
            signer,
            fully_signed,
        });
    }

    /// Make template public in marketplace
    public entry fun publish_to_marketplace(
        template: &mut ContractTemplate,
        marketplace: &mut TemplateMarketplace,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(sender == template.creator, E_NOT_AUTHORIZED);

        template.is_public = true;
        let template_id = object::uid_to_address(&template.id);

        if (!vector::contains(&marketplace.templates, &template_id)) {
            vector::push_back(&mut marketplace.templates, template_id);
        };
    }

    /// Get template info (read-only accessors)
    public fun get_template_name(template: &ContractTemplate): String {
        template.name
    }

    public fun get_template_price(template: &ContractTemplate): u64 {
        template.price_per_use
    }

    public fun get_template_usage_count(template: &ContractTemplate): u64 {
        template.usage_count
    }

    public fun is_instance_signed(instance: &ContractInstance): bool {
        instance.status == 2
    }

    #[test_only]
    public fun test_init(ctx: &mut TxContext) {
        init(ctx);
    }
}
