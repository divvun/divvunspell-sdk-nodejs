"use strict";

const assert = require("assert");
const SpellChecker = require("..");

describe("SpellChecker", () => {
  it("should be able to create a new instance", () => {
    const sc = new SpellChecker("test/sma-mobile.zhfst");
    assert.ok(sc);
  });

  it("should be able to get locale", () => {
    const sc = new SpellChecker("test/sma-mobile.zhfst");
    assert.equal(sc.locale, "sma");
  });

  it("should be able to get locale name", () => {
    const sc = new SpellChecker("test/sma-mobile.zhfst");
    assert.equal(
      sc.localeName,
      "Giellatekno/Divvun/UiT fst-based speller for Southern Sami"
    );
  });

  it("should be able to check if a word is incorrect", async () => {
    const sc = new SpellChecker("test/sma-mobile.zhfst");
    const res = await sc.isCorrect("basse");
    assert.ok(!res);
  });

  it("should be able to check if a word is correct", async () => {
    const sc = new SpellChecker("test/sma-mobile.zhfst");
    const res = await sc.isCorrect("jïh");
    assert.ok(res);
  });

  it("should be able to suggest corrections for an incorrect word", async () => {
    const sc = new SpellChecker("test/sma-mobile.zhfst");
    const res = await sc.suggest("basse");
    assert.ok(res.length > 0);
    // check that the result is an array
    assert.ok(Array.isArray(res));
    // check that the content of the array are strings
    res.forEach((s) => assert.equal(typeof s, "string"));
  });

  it("should be able to suggest corrections for a word", async () => {
    const sc = new SpellChecker("test/sma-mobile.zhfst");
    const res = await sc.suggest("jïh");
    assert.ok(res.length > 0);
  });
});
