import { Component, OnInit } from '@angular/core';
import { BlockService } from '../block.service';

@Component({
  selector: 'app-block-list',
  templateUrl: './block-list.component.html',
  styleUrls: ['./block-list.component.css']
})
export class BlockListComponent implements OnInit {
  heights: number[] = [];

  constructor(private blockService: BlockService) { }

  ngOnInit(): void {
    this.blockService.getHeights().subscribe(heights => {
      this.heights = heights;
    });
  }
}
