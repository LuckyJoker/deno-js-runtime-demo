console.log("Hello", "runjs!");
console.error("Boom");

const path = "./log.txt";
await runjs.writeFile(path, "Hello World!");
try {
    const contents = await runjs.readFile(path);
    console.log("Read from a file", contents);
} catch (err) {
    console.error("Unable to read a file", path, err);
}

await runjs.writeFile(path, "I can write to a file.");
const contents = await runjs.readFile(path);
console.log("Read from a file", path, "contents:", contents);
console.log("Removing file", path);
runjs.removeFile(path);
console.log("File removed");

let content = await runjs.fetch("https://jsonplaceholder.typicode.com/todos/1");
console.log("Content from fetch", content);