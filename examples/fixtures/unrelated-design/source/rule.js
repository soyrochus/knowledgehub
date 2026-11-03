// Harmless retained sentinel; importing must never call this entrypoint.
export function bookingPolicy() { throw new Error('STORED_RULE_MUST_NOT_EXECUTE'); }
