const express = require('express');
const path = require('path');

const app = express();
const PORT = 8000;
const DIRECTORY = path.join(__dirname, "src/assets");

app.use(express.static(DIRECTORY));

app.listen(PORT, () => console.log(`Server listening on port ${PORT}`));

