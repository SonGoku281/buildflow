# BuildFlow Payment

Pluggable payment gateway adapters.

## Architecture

```
PaymentService (interface)
├── RazorpayAdapter
├── JuspayAdapter
└── CustomAdapter
```

## Interface

```typescript
interface PaymentService {
  createOrder(amount: number, currency: string): Promise<Order>;
  verifyPayment(orderId: string, paymentId: string): Promise<boolean>;
  refundPayment(paymentId: string): Promise<Refund>;
  webhookHandler(payload: any): Promise<void>;
}
```
