/* eslint-disable @typescript-eslint/no-var-requires */
const path = require('path')
const iTunesImport = require('./import_itunes.js')
window.addon = require('../../build/addon.node')
const { pathToFileURL } = require('url')

window.toFileUrl = (...args) => {
  const combinedPath = path.join(...args)
  return pathToFileURL(combinedPath).href
}

window.iTunesImport = iTunesImport
