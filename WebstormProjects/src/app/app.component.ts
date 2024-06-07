import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';

interface Record {
  id?: number;
  height: number;
  name?: string;
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  records: Record[] = [];

  constructor(private http: HttpClient) {}

  ngOnInit(): void {
    this.http.get<Record[]>('http://localhost:8000/data').subscribe(data => {
      this.records = data;
    });
  }
}
