import * as fs from "fs";

const complexObject = {
  user: {
    name: "Alice",
    age: 30,
    addresses: [
      { city: "New York", country: "USA" },
      { city: "Milan", country: "Italy" }
    ]
  },
  order: {
    id: 12345,
    items: ["book", "pen", "laptop"],
    price: 99.99
  }
};

fs.writeFileSync("complexObject.json", JSON.stringify(complexObject, null, 2));
