// Rate limiting middleware for free estimate abuse prevention
// Uses in-memory token bucket for MVP (upgrade to Redis for scale)
//
// Limits:
// - 1 free estimate per IP per 24 hours
// - 5 API requests per IP per minute (general)
