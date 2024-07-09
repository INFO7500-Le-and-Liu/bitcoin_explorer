//----block.service.ts
//----block.service.ts
import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

interface Block {
  hash: string;
  time: string;
  block_index: number;
  height: number;
  fee: number;
  n_tx: number;
}
interface NewsItem {
  id: string;
  title: string;
  url: string;
  body: string;
  source: string;
  tags: string;
}


@Injectable({
  providedIn: 'root'
})
export class BlockService {
  private apiUrl = 'https://bitcoinexploreapi-95fb5674dd27.herokuapp.com/blocks';
  private apiUrl_news = 'https://bitcoinexploreapi-95fb5674dd27.herokuapp.com/news';

  constructor(private http: HttpClient) { }

  getBlocks(): Observable<Block[]> {
    return this.http.get<Block[]>(this.apiUrl);
  }

  getNews(): Observable<NewsItem[]> {
    return this.http.get<NewsItem[]>(this.apiUrl_news);

  }
}
