#!/usr/bin/env node

const fs = require('fs');
const os = require('os');
const path = require('path')
const yargs = require('yargs/yargs');
const {hideBin} = require('yargs/helpers');

const savePath = path.join(os.homedir(), 'u', 'todo.txt');


const argv = yargs(hideBin(process.argv))
.alias({
  n: 'name',
  f: 'finished',
  o: 'on-going'
})
.argv;

function main(){
  // * no keyword
  if(!argv._.length){
    console.log(getTodo(argv.finished, argv['on-going']));
    return;
  }
  const command = argv._[0];
  switch(command){
    case 'add': {
      const todoName = argv.name;
      newTodo(todoName);
      return;
    }
    case 'clear': {
      clearTodo();
      return;
    }
    case 'done': {
      const todoName = argv.name;
      finishTodo(todoName);
      return;
    }
  }
}

function newTodo(name){
  if(!name){
    return;
  }
  // ! comma causes parsing issue
  if(name.includes(',')){
    console.log('cannot include "," in name');
    return;
  }

  const data = `${name}, \n`;
  try {
    fs.appendFileSync(savePath, data); 
  }
  catch(err){
    // * if err assume that 
    // * folder u does not exist yet
    // * so create folder first
    fs.mkdirSync(path.join(os.homedir(), 'u'));
    fs.appendFileSync(savePath, data);
    return;
  }
}

function getTodo(finished, onGoing){
  let data = ''
  try{
    data = fs.readFileSync(savePath, 'utf-8');
    if(finished){
      const fltrCond = cells => cells.length > 1 && cells[1] === 'done'
      data = data
      .split('\n')
      .map(line => line.split(', '))
      .filter(fltrCond)
      .map(cells => cells[0])
      .join('\n');
    }

    // * on going takes precedence
    if(onGoing){
      const fltrCond = cells => cells.length > 1 && cells[1] !== 'done'
      data = data
      .split('\n')
      .map(line => line.split(', '))
      .filter(fltrCond)
      .map(cells => cells[0])
      .join('\n');
    }
  }catch(err){
    
  }
  return data;
}

function clearTodo(){
  fs.writeFileSync(savePath, '', 'utf-8');
}

function finishTodo(name){
  let data = fs.readFileSync(savePath, 'utf-8');
  data = data
  .split('\n')
  .map(line => line.split(','))
  .map(cells => {
    if(cells[0] === name){
      cells[1] = ' done, '
    }
    return cells;
  })
  .join('\n');
  fs.writeFileSync(savePath, data, 'utf-8');
}

main();