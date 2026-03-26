# Selection and Interaction Model

## Selection classes
- entity/object selection
- component/sub-object selection
- terrain/brush selection
- graph/node selection
- review/annotation selection

## Interaction classes
- click select
- box/lasso select
- transform manipulator interaction
- drag and drop
- contextual invoke
- modal tool interaction
- staged apply/confirm/cancel interaction

## Laws
- lower-stack truth mutation never occurs directly from pointer input
- user interaction is normalized through `L8.8`
- selection presentation state is not transaction authority
- focused interaction owns active manipulator routing; all others are passive
