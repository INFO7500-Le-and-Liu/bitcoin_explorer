import { HttpClient } from '@angular/common/http';
import { Component, OnInit, OnDestroy } from '@angular/core';
import { BlockService } from './block.service';
import { Subscription } from 'rxjs';

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
    this.http.get<Block[]>('https://bitcoinexploreapi-95fb5674dd27.herokuapp.com/blocks').subscribe(data => {
      this.blocks = data;
    });
  }
}
