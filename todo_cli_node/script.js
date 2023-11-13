const fs = require('fs')
const os = require('os')
const path = require('path')

const savePath = path.join(os.homedir(), 'u', 'todo.txt');
function adjustLine(line){
  return line.replace(' ,', '')
}

const data = fs.readFileSync(savePath, 'utf-8')
.split('\n')
// .map(line => adjustLine(line))
.filter(line => !line.startsWith('e2e'))
.join('\n');

fs.writeFileSync(savePath, data, 'utf-8');