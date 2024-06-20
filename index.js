const {
  createSpellChecker,
  suggest,
  isCorrect,
  locale,
  localeName,
} = require("./index.node");

class SpellChecker {
  constructor(path) {
    this.boxed = createSpellChecker(path);
  }

  suggest(word) {
    return suggest(this.boxed, word);
  }

  isCorrect(word) {
    return isCorrect(this.boxed, word);
  }

  get locale() {
    return locale(this.boxed);
  }

  get localeName() {
    return localeName(this.boxed);
  }
}

module.exports = SpellChecker;
