# package_market_and_dependency_service Level

Canonical layer: `package_market_and_dependency_service`
Activation class: `cold-service`.

## Owns
- package feeds, dependency resolution views, version health, package import/export, and package trust surfaces

## Consumes
- content browser, build surfaces, and governance meta

## Emits
- package dependency and acquisition requests

## Never owns
- runtime authority
