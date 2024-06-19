import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';

interface Block {
  hash: string;
  time: string;
  block_index: number;
  height: number;
  fee: number;
  n_tx: number;
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'blockchain-web';
  blocks: Block[] = [];

  constructor(private http: HttpClient) {}

  ngOnInit(): void {
    this.http.get<Block[]>('http://0.0.0.0:8080/blocks').subscribe(data => {
      this.blocks = data;
    });
  }
}
