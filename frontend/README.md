# Frontend

Currently the Frontend doesn't interact with the backend.
Only a mock connection is available which increases Mana, chooses random other nodes,
and serves pages from `assets/staticPages`.
To run the frontend, you need to start two commands:

```bash
bun watchMock &
bun serve.js
```

The `bun start` doesn't work, as it tries to serve pages like `/page/cybernode/index.html` directly from the
file-tree, instead of calling the component.

If you modify the files in the assets-tree, you need to re-start `bun watchMock`.