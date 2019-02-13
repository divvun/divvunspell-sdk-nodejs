var addon = require('../native');
const NativeSpellChecker = addon.SpellChecker

class SpellChecker {
  constructor(path) {
    this._native = new NativeSpellChecker(path)
  }

  suggest(word) {
    return new Promise((resolve, reject) => {
      this._native.suggest(word, (err, res) => {
        if (err) {
          reject(err)
        } else {
          resolve(res)
        }
      })
    })
  }

  isCorrect(word) {
    return new Promise((resolve, reject) => {
      this._native.isCorrect(word, (err, res) => {
        if (err) {
          reject(err)
        } else {
          resolve(res)
        }
      })
    })
  }

  get locale() {
    return this._native.locale()
  }

  get localeName() {
    return this._native.localeName()
  }
}

module.exports = SpellChecker
