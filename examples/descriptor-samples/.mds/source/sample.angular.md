# Angular Descriptor Sample

## Purpose

Verify that the Angular overlay descriptor can render a source file.

## Contract

- `DescriptorSampleComponent` exposes one template.

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| external | @angular/core | Component | - | Angular component decorator. | - |

## Source

```ts
@Component({
  selector: 'descriptor-sample',
  template: '<p>ok</p>',
})
export class DescriptorSampleComponent {}
```
