import { Paragraph } from "./paragraph";

export class Comment {
  id: number;
  _author: string;
  _date: string;
  _paragraph: Paragraph;

  constructor(id: number) {
    this.id = id;
  }

  author(author: string) {
    this._author = author;
    return this;
  }

  date(date: string) {
    this._date = date;
    return this;
  }

  paragraph(p: Paragraph) {
    this._paragraph = p;
    return this;
  }
}
