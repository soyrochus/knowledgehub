# FruitTrucks process brief

> Authored mockup summary of course/domain/fruittrucks.md and fruittrucks-process.md, not an original client document.

FruitTrucks distributes fresh produce using its refrigerated fleet. Dispatchers need drivers to understand route changes and customer service to see the effective delivery plan.

## Existing requirements

**R8:** A route change after departure is valid only once the driver acknowledges it. **Q1:** The instruction reaches the driver’s phone within 60 seconds. This is a delivery target, not an acknowledgement deadline.

The operations manual requires acknowledgement; the dispatcher interview reports a different practice. Retain both sources rather than silently accepting that practice as policy.

## Proposed FT-P01 subset

Reorder only unserved stops. Keep driver, vehicle, cargo, orders and completed deliveries unchanged. Dispatch owns the effective plan; Delivery Execution supplies exact responses and progress; Customer Communication tracks information about the established plan.

Loom models these definitions. It does not dispatch vehicles or send messages.