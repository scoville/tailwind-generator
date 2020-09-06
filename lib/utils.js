"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.readClasses = exports.shutDownLog = exports.isValidLang = exports.camelCase = void 0;
var fs_1 = __importDefault(require("fs"));
var postcss_1 = __importDefault(require("postcss"));
var langs_1 = __importDefault(require("./langs"));
exports.camelCase = function (name) {
    var _a = [
        name.substr(0, 1),
        name.substr(1, name.length - 1),
    ], _b = _a[0], head = _b === void 0 ? "" : _b, _c = _a[1], tail = _c === void 0 ? "" : _c;
    return "" + head.toLowerCase() + tail.replace(/[-_\s]+([a-z0-9])/g, function (_, word) {
        return word.toUpperCase();
    });
};
exports.isValidLang = function (str) {
    return langs_1.default.includes(str);
};
exports.shutDownLog = function (f) { return __awaiter(void 0, void 0, void 0, function () {
    var oldConsoleLog, res;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                oldConsoleLog = console.log;
                // tslint:disable-next-line: no-console
                console.log = function () { return undefined; };
                return [4 /*yield*/, f()];
            case 1:
                res = _a.sent();
                // tslint:disable-next-line: no-console
                console.log = oldConsoleLog;
                return [2 /*return*/, res];
        }
    });
}); };
var classNameSimpleRegExp = /\.-?[_a-zA-Z]+[\:\\_a-zA-Z0-9-]*/;
var pseudoClasses = "(\n  \\:after\n  |\\:before\n  |\\:focus\n  |\\:hover\n  |\\:active\n  |\\:disabled\n  |\\:visited\n  |\\:first-child\n  |\\:last-child\n  |\\:\\:placeholder\n  |\\:\\:\\-ms\\-input\\-placeholder\n  |\\:\\-ms\\-input\\-placeholder\n  |\\:\\:\\-moz\\-placeholder\n  |\\:\\:\\-webkit\\-input\\-placeholder)$\n";
var pseudoClassesRegExp = new RegExp(pseudoClasses
    .split("\n")
    .map(function (s) { return s.trim(); })
    .join("")
    .trim());
var removePseudoClasses = function (className) {
    while (pseudoClassesRegExp.test(className)) {
        className = className.replace(pseudoClassesRegExp, "");
    }
    return className;
};
var extractClassNameFromSelector = function (selector) {
    var matches = selector.match(classNameSimpleRegExp);
    if (!matches) {
        return;
    }
    var className = matches[0];
    if (!className) {
        return;
    }
    return removePseudoClasses(className
        .trim()
        .replace(/^\./, "")
        .replace(/\\\//g, "/")
        .replace(/\\/g, ""));
};
exports.readClasses = function (filepath) {
    var root = postcss_1.default.parse(fs_1.default.readFileSync(filepath, "utf8"));
    var classes = [];
    root.walkRules(function (_a) {
        var selector = _a.selector;
        // Ignore anything that's not a class
        var className = extractClassNameFromSelector(selector);
        // Skip if it already exists, or if it's not a class selector
        if (!className || classes.some(function (c) { return c.className === className; })) {
            return;
        }
        var name = exports.camelCase(className
            .replace(/\\\//g, "Over")
            .replace(/\\/g, "")
            .replace(/^\-/, "neg-")
            .replace(/\:\-/, "-neg-")
            .replace(/:/g, "-"));
        classes.push({
            className: className,
            name: name,
        });
    });
    return classes;
};
