import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Record } from './record.model';

// interface Record {
//   height: number;
// }

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  records: Record[] = [];

  constructor(private http: HttpClient) {}

  ngOnInit(): void {
    this.http.get<Record[]>('http://localhost:8080/blocks').subscribe(data => {
      this.records = data;
    });
  }
}
