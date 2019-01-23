import childProcess from 'child_process';
import fs from 'fs';
import os from 'os';
import util from 'util';

const execFile = util.promisify(childProcess.execFile);
const readFile = util.promisify(fs.readFile);
const sysname = os.type();

switch (sysname) {
  case "Darwin":
  case "FreeBSD":
    execFile("sysctl", ["vm.loadavg"]).then(({stdout}) => {
      console.log(stdout.split(' ').slice(2, 5).join(' '));
    });
    break;

  case "Linux":
    readFile("/proc/loadavg", "utf8").then((output) => {
      console.log(output.split(' ').slice(0, 3).join(' '));
    });
    break;

  default:
    console.log(`Unknown system: ${sysname}`);
}
