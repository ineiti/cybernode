# Frontend

To run the frontend, you need to start two commands:

```bash
bun watch &
bun serve.js
```

The `bun start` doesn't work, as it tries to serve pages like `/page/cybernode.html` directly from the
file-tree, instead of calling the component.

If you modify the files in the assets-tree, you need to re-start `bun watch`.