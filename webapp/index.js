const express = require("express");
const path = require("path");

const PORT = 80;

const app = express();
app.use(function (req, res, next) {
  console.log(`${req.method} ${req.url}`);
  next();
});
app.use("/", express.static(__dirname));
app.use("/go", express.static(path.join(__dirname, "..", "assemblyscript")));
app.use(
  "/assemblyscript",
  express.static(path.join(__dirname, "..", "assemblyscript"))
);

app.listen(PORT);
console.log(`Running on http://localhost:${PORT}`);
