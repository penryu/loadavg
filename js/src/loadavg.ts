#!/usr/bin/env node

import childProcess from 'child_process';
import os from 'os';
import util from 'util';

const exec = util.promisify(childProcess.exec);

const getOutput = async (command: string) => {
  const { stdout: out, stderr: err } = await exec(command);

  if (err) {
    console.warn(`stderr: ${err}`);
  }

  return out;
};

const getloadavg = async () => {
  const sysname = os.type();

  switch (sysname) {
    case "Darwin":
      const sysctlOut = await getOutput("sysctl vm.loadavg");
      const values = sysctlOut.split(/\s+/).slice(2, 5);
      return values;
      break;

    case "Linux":

    default:
      console.log(`Unknown system: ${sysname}`);
  }

  return [1, 2, 3];
};

const main = async () => {
  const loads = await getloadavg();
  console.log(loads.join(" "));
};

main();
