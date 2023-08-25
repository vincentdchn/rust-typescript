import fs from 'fs';
fs.readFileSync(process.argv[2])
    .toString()
    .split('\n')
    .forEach((line) => {
        const v = parseInt(line);
        if (isNaN(v)) {
            console.log('Line not a number');
        } else {
            console.log(v);
        }
    });
