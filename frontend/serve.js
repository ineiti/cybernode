const express = require('express');
const path = require('path');

const app = express();
const PORT = 8000;
const DIRECTORY = path.join(__dirname, "dist/frontend/browser");

app.use(express.static(DIRECTORY));

app.use((req, res, next) => {
  console.log(`${req.method} ${req.url}`);
  next();
});

app.use((req, res) => {
  console.log('(404)');
  res.status(202).sendFile(path.join(__dirname, "dist/frontend/browser/index.html"));
});

app.listen(PORT, () => console.log(`Server listening on port ${PORT}`));

